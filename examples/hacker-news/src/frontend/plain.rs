use cached::proc_macro::cached;
use html_strong::document_tree::{o, Node};
use html_strong::tags::*;
use html_strong::template;

use crate::story::Story;
use crate::util;

use super::Renderable;

pub struct Plain;

#[cached]
fn plain_story(story: Story) -> Node {
    let title = o(H1).add_text(&format!("#{} - {}", story.rank, story.title));

    let subtitle = o(P).add_text(&format!(
        "{} points • by {} • {}",
        story.upvotes,
        story.author,
        util::time_ago(story.submission_time),
    ));

    let subtitle = if let Some(url) = story.url {
        let short = url.domain().unwrap_or("link");

        subtitle
            .add_text(" • ")
            .kid(o(A::href(&url.to_string())).add_text(short))
    } else {
        subtitle
    };

    o(Div).add_class("story").kid(title).kid(subtitle)
}

impl Renderable for Plain {
    fn frontpage(&self, stories: Vec<Story>) -> Node {
        let mut body = o(Body);

        for story in stories {
            body.push_kid(plain_story(story));
        }

        template::HtmlDocumentBuilder::new().with_body(body).build()
    }

    fn comments(&self, story: Story) -> Node {
        todo!()
    }
}
