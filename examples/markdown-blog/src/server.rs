use anyhow::{Context, Result};
use axum::{
    extract::Path,
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use examples_lib::internal_server_error;
use tokio::{fs::File, io::AsyncReadExt};
use tower_http::services::ServeDir;

use crate::md_to_html;

async fn read_file(path: String) -> Result<String> {
    let mut f = File::open(&path).await.with_context(|| path)?;
    let mut md = String::new();
    f.read_to_string(&mut md).await?;

    Ok(md)
}

async fn blog_post_impl(blog_post: String) -> Result<Html<String>> {
    // TODO: Find a nicer way to do this. Ideally we would just use
    // "hello-world.md".
    //
    // Could we make this into a request, and use axum features?
    let md = read_file(format!("../markdown-blog/static/{blog_post}.md")).await?;
    let body = md_to_html::md_to_html(&md);

    let doc = examples_lib::html_doc(
        Some(vec![
            // TODO: Fix this prefix
            "/blog/static/css/blog.css",
            "https://cdn.jsdelivr.net/npm/comic-mono@0.0.1/index.css",
            "https://fonts.googleapis.com/css2?family=Domine:wght@500&display=swap",
            "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/styles/monokai.min.css",
        ]),
        Some(vec![
            "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/highlight.min.js",
        ]),
        Some(vec!["hljs.highlightAll();"]),
        body,
    );

    examples_lib::render(doc)
}

async fn blog_post(Path(blog): Path<String>) -> std::result::Result<Html<String>, String> {
    blog_post_impl(blog).await.map_err(|e| format!("{e:#?}"))
}

async fn default_blog_post() -> impl IntoResponse {
    Redirect::to("/blog/hello-world")
}

pub struct MarkdownBlog;

impl examples_lib::Example for MarkdownBlog {
    fn router(&self, from_me_to_you: &str) -> Router {
        // Redirectus erectus
        Router::new()
            .route("/", get(default_blog_post))
            .route("/:post", get(blog_post))
            .nest(
                "/static",
                get_service(ServeDir::new(format!("{from_me_to_you}/static")))
                    .handle_error(internal_server_error),
            )
    }
}
