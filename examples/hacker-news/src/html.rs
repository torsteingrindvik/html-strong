use crate::state::SharedState;
use crate::story::Story;

use axum::{
    extract::{Extension, Query},
    response::Html,
};
use cached::proc_macro::cached;
use html_strong::{
    document_tree::{o, Node},
    global_attributes::Lang,
    tags::{
        form, html, td::td, Body, Br, Div, Form, Head, Img, Input, Link, Meta, Script, Span, Table,
        Td, Title, Tr, A, B,
    },
};
use serde::Deserialize;
use std::time::Instant;
use tracing::debug;

fn get_response(contents: Node) -> Html<String> {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Html(response)
}

use crate::constants::*;

fn a(href: &str, text: &str) -> Node {
    o(A::href(href)).add_text(text)
}

fn a2(href_text: &str) -> Node {
    o(A::href(href_text)).add_text(href_text)
}

#[cached]
fn body_nav() -> Node {
    let td_logo = o(td())
        .add_style("width:18px;padding-right:4px;")
        .kid(o(A::href("https://news.ycombinator.com")).kid(
            o(Img::new_sized("/static/y18.gif", 18, 18)).add_style("border:1px white solid;"),
        ));

    let td_links = o(td()).add_style("line-height:12pt; height:10px;").kid(
        o(Span)
            .add_class("pagetop")
            .kid(o(B).add_class("hnname").kid(a("news", "Hacker News")))
            .add_text(ONE_SPACE)
            .kid(a("newest", "new"))
            .add_text(PIPE_DELIMITER)
            .kid(a("newcomments", "comments"))
            .add_text(PIPE_DELIMITER)
            .kid(a2("ask"))
            .add_text(PIPE_DELIMITER)
            .kid(a2("show"))
            .add_text(PIPE_DELIMITER)
            .kid(a2("jobs"))
            .add_text(PIPE_DELIMITER)
            .kid(a2("submit")),
    );

    let td_login = o(td())
        .add_style("text-align:right;padding-right:4px;")
        .kid(
            o(Span)
                .add_class("pagetop")
                .kid(a("login?goto=news", "login")),
        );

    o(Tr).kid(
        o(td()).set_id("nav-td").kid(
            o(Table)
                .set_id("nav-table")
                .add_style("padding:2px")
                .kid(o(Tr).kid(td_logo).kid(td_links).kid(td_login)),
        ),
    )
}

#[cached]
fn body_spacer() -> Node {
    o(Tr)
        .set_id("pagespace")
        .set_title("") // Well ok
        .add_style("height:10px")
}

/// Get the <head>...</head> contents.
/// The title differs on the frontpage vs. the comment page.
///
/// The frontpage wants the RSS alternate element, the comment page does not.
#[cached]
fn head(title: String, add_alternate: bool) -> Node {
    let mut head = o(Head)
        .kid(Meta::name_content("referrer", "origin"))
        .kid(Meta::viewport_sane())
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news.css"))
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news-extra.css"))
        .kid(Link::icon("favicon.ico"));
    if add_alternate {
        head.push_kid(Link::alternate("application/rss+xml", "RSS", "rss"))
    }

    head.kid(o(Title).add_text(&title))
}

fn body_comments(story: Story) -> Node {
    let fatitem = o(Table)
        .add_class("fatitem")
        .kid(
            o(Tr)
                .add_class("athing")
                .set_id(&story.id.to_string())
                .kid(
                    o(Td::default())
                        .add_class("title")
                        .kid(o(Span).add_class("rank")),
                )
                .kid(
                    o(Td::default()).add_class("votelinks").kid(
                        o(A::href("todo :)"))
                            .set_id(&format!("up_{}", story.id))
                            .kid(o(Div).add_class("votearrow").set_title("upvote")),
                    ),
                ),
        )
        .kid(
            o(Tr)
                .kid(o(Td::colspan(2)))
                // TODO: Rest of this thing
                .kid(o(Td::default()).add_class("subtext")),
        );

    o(Tr).kid(o(td()).kid(fatitem))
}

fn body_stories(stories: Vec<Story>) -> Node {
    let mut items = o(Table).add_class("itemlist");

    for (rank, mut story) in stories.into_iter().enumerate() {
        story.rank = Some(rank + 1);
        items.push_kid(story)
    }

    o(Tr).kid(o(td()).kid(items))
}

fn body(body_node: Node) -> Node {
    o(Body).kid(
        o(Table)
            .set_id("hnmain")
            .kid(body_nav())
            .kid(body_spacer())
            .kid(body_node)
            .kid(body_footer()),
    )
}

#[cached]
fn script() -> Node {
    o(Script)
}

#[cached]
fn body_footer() -> Node {
    let invisible_gif = o(Img::new_sized("/static/s.gif", 0, 10));
    let divider = o(Table).kid(o(Tr).kid(td()).set_id("footer-divider"));
    let applications = o(A::href("https://www.ycombinator.com/apply/"))
        .add_text("Applications are open for YC Summer 2022");

    let links = o(Span)
        .add_class("yclinks")
        .kid(a("newsguidelines.html", "Guidelines"))
        .add_text(PIPE_DELIMITER)
        .kid(a("newsfaq.html", "FAQ"))
        .add_text(PIPE_DELIMITER)
        .kid(a("lists", "Lists"))
        .add_text(PIPE_DELIMITER)
        .kid(a("https://github.com/HackerNews/API", "API"))
        .add_text(PIPE_DELIMITER)
        .kid(a("security.html", "Security"))
        .add_text(PIPE_DELIMITER)
        .kid(a("http://www.ycombinator.com/legal/", "Legal"))
        .add_text(PIPE_DELIMITER)
        .kid(a("http://www.ycombinator.com/apply/", "Apply"))
        .add_text(PIPE_DELIMITER)
        .kid(a("mailto:hn@ycombinator.com", "Contact"));

    let search = o(Form::new(form::Method::Get, "//hn.algolia.com/"))
        .add_text("Search: ")
        .kid(o(Input));

    o(Tr).set_id("footer").kid(
        o(td())
            .kid(invisible_gif)
            .kid(divider)
            .kid(Br)
            .kid(applications)
            .kid(Br)
            .kid(Br)
            .kid(links)
            .kid(Br)
            .kid(Br)
            .kid(search),
    )
}

pub async fn front_page(Extension(state): Extension<SharedState>) -> Html<String> {
    let now = Instant::now();
    let stories = state.0.read().await.values().cloned().collect();
    debug!("Stories acquired (held read lock for {:?})", now.elapsed());

    let story_nodes = body_stories(stories);

    get_response(
        o(html::Html)
            .add_attr(("op", "news"))
            .add_attr(Lang::English)
            .kid(head("Hacker News".into(), true))
            .kid(body(story_nodes))
            .kid(script()),
    )
}

#[derive(Debug, Deserialize)]
pub struct Item {
    id: usize,
}

pub async fn comment_page(
    Query(Item { id }): Query<Item>,
    Extension(state): Extension<SharedState>,
) -> Html<String> {
    if let Some(story) = state.0.read().await.get(&id) {
        let title = format!("{} | Hacker News", story.title);

        let comment_nodes = body_comments(story.clone());

        get_response(
            o(html::Html)
                .add_attr(("op", "item"))
                .add_attr(Lang::English)
                .kid(head(title, false))
                .kid(body(comment_nodes))
                .kid(script()),
        )
    } else {
        format!("TODO error handle, missing id {id:?}").into()
    }
}
