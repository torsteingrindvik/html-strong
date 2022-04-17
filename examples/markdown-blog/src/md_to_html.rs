use html_strong::document_tree::Node;
use html_strong::science_lab::NodeExt;
use html_strong::tags::*;
use pulldown_cmark::Event;
use tracing::debug;

/// When parsing markdown, prefix this to any URLs.
/// E.g. `(thing)[some/path]` will end up with the URL `/blog/static/some/path`.
///
/// TODO: This example should not have to know that the server expects all blog-related requests
/// to start with "/blog".
/// Therefore the trait should include that information in some arg instead.
pub const STATIC_PREPEND: &str = "/blog/static";

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
            Tag::Link(LinkType::Inline, url, title) => (
                stack_add(
                    nodes,
                    A::href(&format!("{STATIC_PREPEND}/{url}")).text(title),
                ),
                context,
            ),
            Tag::Image(LinkType::Inline, url, _title) => (
                stack_add(nodes, Img::new(&format!("{STATIC_PREPEND}/{url}"))),
                Some(Context::Image),
            ),
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

pub fn md_to_html(md: &str) -> Node {
    let parser = pulldown_cmark::Parser::new(md);

    let (mut html, _) = parser
        .into_iter()
        .fold((vec![Div.into_node()], None), consume_event);

    let body = html.pop().expect("Root div");
    assert!(
        html.is_empty(),
        "Should end up with only the initial element- kids added"
    );

    body
}
