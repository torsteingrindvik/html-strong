use axum::response::Html;
use axum::Router;
use html_strong::science_lab::NodeExt;
use html_strong::tags;
use html_strong::{document_tree::Node, template};
use reqwest::StatusCode;

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
            head.push_kid(tags::Link::stylesheet(mime::TEXT_CSS, css.as_ref()));
        }
    }

    // Add scripts.
    if let Some(script) = script {
        for script in script {
            head.push_kid(tags::Script::src(script.as_ref()));
        }
    }

    // Add scripts where content is defined inline.
    if let Some(script) = script_inline {
        for script in script {
            head.push_kid(tags::Script::new().text(script));
        }
    }

    template::HtmlDocumentBuilder::new()
        // TODO: <nav>
        .with_head(head)
        .with_body(body)
        // TODO: <footer>
        .build()
}
