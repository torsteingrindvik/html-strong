use cached::proc_macro::cached;
use html_strong::document_tree::Node;
use html_strong::science_lab::NodeExt;
use html_strong::tags::*;

use crate::story::Story;
use crate::util;

use super::Renderable;

pub struct Plain;

#[cached]
fn plain_story(story: Story) -> Node {
    let title = H1.text(format!("#{} - {}", story.rank, story.title));

    let subtitle = P.text(format!(
        "{} points • by {} • {}",
        story.upvotes,
        story.author,
        util::time_ago(story.submission_time),
    ));

    let subtitle = if let Some(url) = story.url {
        let short = url.domain().unwrap_or("link");

        subtitle
            .add_text(" • ")
            .kid(A::href(&url.to_string()).text(short))
    } else {
        subtitle
    };

    Div.class("story").kid(title).kid(subtitle)
}

impl Renderable for Plain {
    fn frontpage(&self, stories: Vec<Story>) -> Node {
        let mut contents = Div.into_node();

        for story in stories {
            contents.push_kid(plain_story(story));
        }

        examples_lib::html_doc::<String>(None, None, None, contents)
    }

    fn comments(&self, _story: Story) -> Node {
        todo!()
    }
}
