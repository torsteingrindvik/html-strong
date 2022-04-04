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
        form::{self, Method},
        html,
        td::td,
        Body, Br, Div, Form, Head, Img, Input, Link, Meta, Script, Span, Table, Td, Textarea,
        Title, Tr, A, B,
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
    let score_id = &format!("score_{}", story.id);
    let unv_id = &format!("unv_{}", story.id);
    let item_id = &format!("item?id={}", story.id);
    let time_ago_href = o(A::href(item_id)).add_text(&story.time_ago());

    let score_span = o(Span)
        .add_class("score")
        .set_id(score_id)
        .add_text(&format!("{} points", story.score));

    let user_href = o(A::href(&format!("user?id={}", story.by)))
        .add_class("hnuser")
        .add_text(&story.by);

    let age_span = o(Span)
        .add_class("age")
        .set_title("2022-TODO")
        .kid(time_ago_href);

    let unv_span = o(Span).set_id(unv_id);
    let hide_a = o(A::href("TODO")).add_text("hide");
    let past_a = o(A::href("TODO")).add_class("hnpast").add_text("past");
    let fav_a = o(A::href("TODO")).add_text("favorite");
    let comments_a = o(A::href(item_id)).add_text(&format!("{} comments", story.descendants));

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
                        o(A::href(&format!(
                            "vote?id={}&amp;how=up&amp;goto=item%3Fid%3D{}",
                            story.id, story.id
                        )))
                        .set_id(&format!("up_{}", story.id))
                        .kid(o(Div).add_class("votearrow").set_title("upvote")),
                    ),
                )
                .kid(
                    o(Td::default())
                        .add_class("title")
                        .kid(
                            o(A::href(
                                &story
                                    .url
                                    .map(|url| url.to_string())
                                    .unwrap_or_else(|| "todo :-)".into()),
                            ))
                            .add_class("titlelink")
                            .add_text(&story.text.unwrap_or_default()),
                        )
                        .kid(
                            o(Span)
                                .add_class("sitebit comhead")
                                .add_text(" (")
                                .kid(
                                    o(A::href("from?site=todo"))
                                        .kid(o(Span).add_class("sitestr").add_text("todo")),
                                )
                                .add_text(")"),
                        ),
                ),
        )
        .kid(
            o(Tr).kid(o(Td::colspan(2))).kid(
                o(Td::default())
                    .add_class("subtext")
                    .kid(score_span)
                    .add_text(" by ")
                    .kid(user_href)
                    .add_text(ONE_SPACE)
                    .kid(age_span)
                    .add_text(ONE_SPACE)
                    .kid(unv_span)
                    .add_text(PIPE_DELIMITER)
                    .kid(hide_a)
                    .add_text(PIPE_DELIMITER)
                    .kid(past_a)
                    .add_text(PIPE_DELIMITER)
                    .kid(fav_a)
                    .add_text(PIPE_DELIMITER)
                    .kid(comments_a),
            ),
        )
        .kid(o(Tr).add_style("height:10px"))
        .kid(
            o(Tr).kid(o(Td::colspan(2))).kid(
                o(Td::default()).kid(
                    o(Form::new(Method::Post, "comment"))
                        .kid(Input::hidden("parent", &story.id.to_string()))
                        .kid(Input::hidden("goto", &format!("item?id={}", story.id)))
                        .kid(Input::hidden("hmac", "hmac-of-what?"))
                        .kid(Textarea::new("text", 8, 80))
                        .kid(Br)
                        .kid(Br)
                        .kid(Input::submit("add comment")),
                ),
            ),
        );

    o(Tr).kid(o(td()).kid(fatitem).kid(Br).kid(Br))
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
        .kid(o(Input::text("q", "")));

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
