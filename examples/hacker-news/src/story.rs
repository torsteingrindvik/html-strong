use cached::proc_macro::cached;
use chrono::{DateTime, Local};
use html_strong::{
    document_tree::{o, Node},
    science_lab::NodeExt,
    tags::{td::td, Div, Span, Td, Tr, A},
};
use serde::Deserialize;
use url::Url;

use crate::{
    comment::Comment,
    constants::{ONE_SPACE, PIPE_DELIMITER},
    util::time_ago,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Story {
    /// Author
    pub author: String,

    /// Story id
    pub id: usize,

    /// Story text, if any
    pub text: String,

    /// Comments
    pub comments: Vec<Comment>,

    /// Upvotes
    pub upvotes: usize,

    /// When it was submitted
    pub submission_time: DateTime<Local>,

    /// Story title
    pub title: String,

    /// Story url, or nothing if there was no external story.
    pub url: Option<Url>,

    /// Not in JSON, will be set by us
    pub rank: Option<usize>,
}

#[cached]
fn spacer() -> Node {
    o(Tr).add_class("spacer").add_style("height:5px")
}

#[cached]
fn story(story: Story) -> Node {
    let title_link = if let Some(url) = &story.url {
        url.to_string()
    } else {
        format!("item?id={}", story.id)
    };

    let mut title = o(td()).add_class("title").kid(
        o(A::href(&title_link))
            .add_class("titlelink")
            .add_text(&story.title),
    );

    if let Some(url) = &story.url {
        let url_long = url.to_string();

        // Get the short version of the domain in a best effort manner.
        let url_short = url
            .domain()
            .map(|short| short.to_string())
            .unwrap_or_else(|| url_long.clone());

        title.push_kid(
            o(Span)
                .add_class("sitebit comhead")
                .add_text(" (")
                .kid(
                    o(A::href(&format!("from?site={}", &url_short)))
                        .kid(o(Span).add_class("sitestr"))
                        .add_text(&url_short),
                )
                .add_text(")"),
        );
    }

    let comment_text = if story.comments.is_empty() {
        "discuss".into()
    } else {
        format!("{} comments", story.comments.len())
    };

    Node::root()
        .kid(
            o(Tr)
                .add_class("athing")
                .set_id(&story.id.to_string())
                .kid(
                    o(td()).add_class("title-rank").kid(
                        o(Span)
                            .add_class("rank")
                            .add_text(&format!("{}.", story.rank.expect("Must set rank"))),
                    ),
                )
                .kid(
                    o(td()).add_class("votelinks").kid(
                        o(A::href(&format!("vote?id={}&how=up&goto=news", story.id)))
                            .set_id(&format!("up_{}", story.id))
                            .kid(o(Div).add_class("votearrow").set_title("upvote")),
                    ),
                )
                .kid(title),
        )
        .kid(
            o(Tr).kid(Td::colspan(2)).kid(
                o(td())
                    .add_class("subtext")
                    .kid(o(Span).add_text(&format!("{} points", story.upvotes)))
                    .add_text(" by ")
                    .kid(
                        o(A::href(&format!("user?id={}", story.author)))
                            .add_class("hnuser")
                            .add_text(&story.author)
                            .add_text(ONE_SPACE),
                    )
                    .kid(
                        o(Span)
                            .add_class("age")
                            .set_title("2022-03-28T16:35:29") // TODO: This thing
                            .kid(
                                o(A::href(&format!("item?id={}", story.id)))
                                    .add_text(&time_ago(story.submission_time)),
                            ),
                    )
                    .kid(Span)
                    .add_text(PIPE_DELIMITER)
                    .kid(o(A::href(&format!("hide?id={}&goto=news", story.id))).add_text("hide"))
                    .add_text(PIPE_DELIMITER)
                    .kid(o(A::href(&format!("item?id={}", { story.id }))).add_text(&comment_text)),
            ),
        )
}

impl NodeExt for Story {
    fn into_node(self) -> Node {
        Node::root().kid(story(self)).kid(spacer())
    }
}
