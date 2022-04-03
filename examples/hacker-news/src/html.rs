use crate::state::SharedState;
use crate::story::Story;

use axum::{extract::Extension, response::Html};
use cached::proc_macro::cached;
use html_strong::{
    document_tree::{o, Node},
    global_attributes::Lang,
    tags::{
        form, html, td::td, Body, Br, Form, Head, Img, Input, Link, Meta, Script, Span, Table,
        Title, Tr, A, B,
    },
};
use std::time::Instant;
use tracing::info;

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

#[cached]
fn head() -> Node {
    o(Head)
        .kid(Meta::name_content("referrer", "origin"))
        .kid(Meta::viewport_sane())
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news.css"))
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news-extra.css"))
        .kid(Link::icon("favicon.ico"))
        .kid(Link::alternate("application/rss+xml", "RSS", "rss"))
        .kid(o(Title).add_text("Hacker News"))
}

fn body_stories(stories: Vec<Story>) -> Node {
    let mut items = o(Table).add_class("itemlist");

    for (rank, mut story) in stories.into_iter().enumerate() {
        story.rank = Some(rank + 1);
        items.push_kid(story)
    }

    o(Tr).kid(o(td()).kid(items))
}

#[cached]
fn body(stories: Vec<Story>) -> Node {
    o(Body).kid(
        o(Table)
            .set_id("hnmain")
            .kid(body_nav())
            .kid(body_spacer())
            .kid(body_stories(stories))
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

pub async fn hacker_news(Extension(state): Extension<SharedState>) -> Html<String> {
    let now = Instant::now();
    let stories = state.0.read().await.clone();
    info!("Stories acquired (held read lock for {:?})", now.elapsed());

    get_response(
        o(html::Html)
            .add_attr(("op", "news"))
            .add_attr(Lang::English)
            .kid(head())
            .kid(body(stories))
            .kid(script()),
    )
}
