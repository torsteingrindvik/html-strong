use anyhow::{Context, Result};
use axum::{
    response::Html,
    routing::{get, get_service},
    Router,
};
use examples_lib::internal_server_error;
use tokio::{fs::File, io::AsyncReadExt};
use tower_http::services::ServeDir;

use crate::md_to_html;

async fn read_file(path: &str) -> Result<String> {
    let mut f = File::open(path).await.with_context(|| path.to_string())?;
    let mut md = String::new();
    f.read_to_string(&mut md).await?;

    Ok(md)
}

async fn front_page_impl() -> Result<Html<String>> {
    // TODO: Find a nicer way to do this. Ideally we would just use
    // "hello-world.md".
    //
    // Could we make this into a request, and use axum features?
    let md = read_file("../markdown-blog/static/hello-world.md").await?;
    let body = md_to_html::md_to_html(&md);

    let doc = examples_lib::html_doc(
        Some(vec![
            "/blog/css/blog.css",
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

async fn front_page() -> std::result::Result<Html<String>, String> {
    front_page_impl().await.map_err(|e| format!("{e:#?}"))
}

pub struct MarkdownBlog;

impl examples_lib::Example for MarkdownBlog {
    fn router(&self, from_me_to_you: &str) -> Router {
        Router::new().route("/", get(front_page)).nest(
            "/static",
            get_service(ServeDir::new(format!("{from_me_to_you}/static")))
                .handle_error(internal_server_error),
        )
    }
}
