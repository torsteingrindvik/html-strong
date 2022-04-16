use axum::{
    response::Html,
    routing::{get, get_service},
    Router,
};
use axum_extra::extract::CookieJar;
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*, template};
use pulldown_cmark::Event;
use reqwest::StatusCode;
use std::{borrow::Cow, net::SocketAddr};
use tokio::{fs::File, io::AsyncReadExt};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::debug;

type Result = std::result::Result<(CookieJar, Html<String>), Cow<'static, str>>;

fn get_response(contents: Node, jar: CookieJar) -> Result {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Ok((jar, Html(response)))
}

enum Context {
    // The start event pushed two elements to the stack instead of one.
    TwoElements,

    // We have started an image tag.
    // The text paragraph right after is the alt text,
    // and should not be considered a regular paragraph.
    Image,
}

fn stack_add<N>(mut nodes: Vec<Node>, nodelike: N) -> Vec<Node>
where
    N: NodeExt,
{
    nodes.push(nodelike.into_node());
    nodes
}

// It has cooked long enough!
fn birth(mut nodes: Vec<Node>) -> Vec<Node> {
    // Pop the now completed child..
    let child = nodes.pop().expect("Pop child");
    // ..also need a reference to the parent
    let parent = nodes.pop().expect("Pop parent");

    // Add the parent back to the stack with the newly added child.
    stack_add(nodes, parent.kid(child))
}

fn consume_event(
    baggage: (Vec<Node>, Option<Context>),
    event: Event,
) -> (Vec<Node>, Option<Context>) {
    use pulldown_cmark::{CodeBlockKind, HeadingLevel, LinkType, Tag};

    let (mut nodes, context) = baggage;

    debug!(?event, "Handling event");

    match event {
        // Something is starting.
        // This means we should be adding something to our stack.
        Event::Start(e) => match e {
            Tag::Paragraph => (stack_add(nodes, P), context),
            Tag::Heading(level, _, _) => match level {
                HeadingLevel::H1 => (stack_add(nodes, H1), context),
                HeadingLevel::H2 => (stack_add(nodes, H2), context),
                level => {
                    todo!("Heading level not implememented: {level:?}");
                }
            },
            Tag::BlockQuote => (stack_add(nodes, Blockquote::new()), context),
            Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                // This starts a single event, but introduces two elements to the HTML!
                // Therefore we must also pop twice when this event ends.
                let nodes = stack_add(nodes, Pre);
                let nodes = stack_add(nodes, Code.class(format!("language-{lang}")));

                // The context ensures we don't forget the double elements.
                (nodes, Some(Context::TwoElements))
            }
            Tag::Link(LinkType::Inline, url, title) => {
                (stack_add(nodes, A::href(&url).text(title)), context)
            }
            Tag::Image(LinkType::Inline, url, _title) => {
                (stack_add(nodes, Img::new(&url)), Some(Context::Image))
            }
            others => {
                debug!(?others, "Tag not handled");
                (nodes, context)
            }
        },
        // Something is ending.
        // This means it's now ready to be added to the parent as a completed child.
        // Context is also cleared.
        Event::End(_) => {
            if let Some(Context::TwoElements) = context {
                let nodes = birth(nodes);
                (birth(nodes), None)
            } else {
                (birth(nodes), None)
            }
        }

        // We have text.
        // This means we should add it to the current element in progress.
        Event::Text(text) => {
            let node = nodes.pop().expect("Node");

            if let Some(Context::Image) = context {
                (
                    stack_add(nodes, node.add_attr(("alt", text.to_owned().to_string()))),
                    context,
                )
            } else {
                (stack_add(nodes, node.text(text)), context)
            }
        }

        Event::SoftBreak => {
            let parent = nodes.pop().expect("Pop parent");
            (stack_add(nodes, parent.kid(Br)), context)
        }
        others => {
            debug!(?others, "Event not handled");
            (nodes, context)
        }
    }
}

async fn front_page(jar: CookieJar) -> Result {
    // let now = Instant::now();

    let mut f = File::open("static/blog/hello-world.md")
        .await
        .expect("Should be able to open");
    let mut md = String::new();
    f.read_to_string(&mut md)
        .await
        .expect("Should be able to read");

    let parser = pulldown_cmark::Parser::new(&md);

    let (mut html, _) = parser
        .into_iter()
        .fold((vec![Div.into_node()], None), consume_event);

    let body = html.pop().expect("Root div");
    assert!(
        html.is_empty(),
        "Should end up with only the initial element- kids added"
    );

    let doc = template::HtmlDocumentBuilder::new()
        .with_head(
            template::head()
                .kid(Link::stylesheet(mime::TEXT_CSS, "/static/css/blog.css"))
                .kid(Link::stylesheet(
                    mime::TEXT_CSS,
                    "https://cdn.jsdelivr.net/npm/comic-mono@0.0.1/index.css",
                ))
                .kid(Link::stylesheet(
                    mime::TEXT_CSS,
                    "https://fonts.googleapis.com/css2?family=Domine:wght@500&display=swap",
                ))
                .kid(Link::stylesheet(
                    mime::TEXT_CSS,
                    "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/styles/monokai.min.css",
                ))
                .kid(Script::src(
                    "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/highlight.min.js",
                ))
                .kid(Script::new().text("hljs.highlightAll();")),
        )
        .with_body(body)
        .build();

    get_response(doc, jar)
}

async fn internal_server_error(error: std::io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(front_page))
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("static/favicon.ico")).handle_error(internal_server_error),
        )
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(internal_server_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new()),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
