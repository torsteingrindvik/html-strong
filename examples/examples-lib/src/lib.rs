use axum::{
    response::Html,
    routing::{get, get_service},
    Router,
};
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*, template};
use reqwest::StatusCode;
use tower_http::services::ServeDir;

/// Common functionality in examples..
pub trait Example {
    /// The relative path from me (the server) to
    /// you (the example).
    ///
    /// When serving directories, this should be prepended
    /// such that paths resolve correctly.
    ///
    /// There is no trailing slash in `from_me_to_you`.
    fn router(&self, from_me_to_you: &str) -> Router;
}

pub async fn internal_server_error(error: std::io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}

pub fn render(contents: Node) -> Result<Html<String>, anyhow::Error> {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Ok(Html(response))
}

async fn home() -> Result<Html<String>, String> {
    let contents = Div.kid(
        P.text("Currently hosting ")
            .kid(
                A::href("https://github.com/torsteingrindvik/html-strong/tree/examples-server")
                    .text("html-strong"),
            )
            .text(" on a WIP branch, so lots of jank. Enjoy the jumping navbar and font magic."),
    );
    let html = html_doc(Some(vec!["/static/example.css"]), None, None, contents);

    render(html).map_err(|e| format!("{e:#?}"))
}

pub struct Home;

impl Example for Home {
    fn router(&self, from_me_to_you: &str) -> Router {
        Router::new().route("/", get(home)).nest(
            "/static",
            get_service(ServeDir::new(format!("{from_me_to_you}/static")))
                .handle_error(internal_server_error),
        )
    }
}

/// Wrap contents of example in a common HTML document template.
/// This template mainly sets up a default document structure,
/// and includes the common <nav> (TODO) and <footer> (TODO).
///
/// The optional args inserts things into the header:
///
/// css: A list of paths to any stylesheets to include.
/// script: An optional list of paths to any scripts to include.
/// script_inline: An optional list of inline javascript to include.
///
/// body: The body of the example in question.
/// Note that this function will wrap the passed body node like this:
///
/// <body>
/// <nav>...</nav>
/// <your body></your body>
/// </body>
///
/// So don't actually pass a `Body`.
pub fn html_doc<S: AsRef<str>>(
    css: Option<Vec<S>>,
    script: Option<Vec<S>>,
    script_inline: Option<Vec<S>>,
    body: Node,
) -> Node {
    // Use html-strong's base head template.
    let mut head = template::head();

    // Add stylesheets.
    if let Some(css) = css {
        for css in css {
            head.push_kid(Link::stylesheet(mime::TEXT_CSS, css.as_ref()));
        }
    }

    // Always want the "base CSS" used for the top nav.
    head.push_kid(Link::stylesheet(mime::TEXT_CSS, "/static/example.css"));

    // Add scripts.
    if let Some(script) = script {
        for script in script {
            head.push_kid(Script::src(script.as_ref()));
        }
    }

    // Add scripts where content is defined inline.
    if let Some(script) = script_inline {
        for script in script {
            head.push_kid(Script::new().text(script));
        }
    }

    /*
        See: https://www.w3schools.com/howto/howto_js_topnav.asp

        <div class="topnav">
          <a class="active" href="#home">Home</a>
          <a href="#news">News</a>
          <a href="#contact">Contact</a>
          <a href="#about">About</a>
        </div>

        Not sure why they don't use a <nav>.
    */
    let nav = Nav
        .kid(A::href("/").text("Home"))
        .kid(A::href("/hn").text("Hacker News"))
        .kid(A::href("/blog").text("Blog"))
        .kid(A::href("/hn/settings").text("Settings"));

    let body = Body
        .kid(nav.class("example-nav"))
        .kid(body.class("example-body"));

    template::HtmlDocumentBuilder::new()
        .with_head(head)
        .with_body(body)
        // TODO: <footer>
        .build()
}
