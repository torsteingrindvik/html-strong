use super::Renderable;

pub struct Original;

use crate::comment::Comment;
use crate::story::Story;
use crate::util::time_ago;

use cached::proc_macro::cached;
use html_strong::{
    document_tree::{o, Node},
    global_attributes::Lang,
    science_lab::NodeExt,
    tags::{
        form::{self, Method},
        html,
        td::td,
        Body, Br, Div, Form, Head, Img, Input, Link, Meta, Script, Span, Table, Td, Textarea,
        Title, Tr, A, B, P, U,
    },
};
use tracing::debug;

use crate::constants::*;

fn a(href: &str, text: &str) -> Node {
    A::href(href).text(text)
}

fn a2(href_text: &str) -> Node {
    A::href(href_text).text(href_text)
}

#[cached]
fn body_nav() -> Node {
    let td_logo = td().style("width:18px;padding-right:4px;").kid(
        A::href("https://news.ycombinator.com")
            .kid(Img::new_sized("/static/y18.gif", 18, 18).style("border:1px white solid;")),
    );

    let td_links = td().style("line-height:12pt; height:10px;").kid(
        Span.class("pagetop")
            .kid(B.class("hnname").kid(a("news", "Hacker News")))
            .text(ONE_SPACE)
            .kid(a("newest", "new"))
            .text(PIPE_DELIMITER)
            .kid(a("newcomments", "comments"))
            .text(PIPE_DELIMITER)
            .kid(a2("ask"))
            .text(PIPE_DELIMITER)
            .kid(a2("show"))
            .text(PIPE_DELIMITER)
            .kid(a2("jobs"))
            .text(PIPE_DELIMITER)
            .kid(a2("submit")),
    );

    let td_login = td()
        .style("text-align:right;padding-right:4px;")
        .kid(Span.class("pagetop").kid(a("login?goto=news", "login")));

    Tr.kid(
        td().id("nav-td").kid(
            Table
                .id("nav-table")
                .style("padding:2px")
                .kid(Tr.kid(td_logo).kid(td_links).kid(td_login)),
        ),
    )
}

#[cached]
fn body_spacer() -> Node {
    Tr.id("pagespace")
        .set_title("") // Well ok
        .style("height:10px")
}

/// Get the <head>...</head> contents.
/// The title differs on the frontpage vs. the comment page.
///
/// The frontpage wants the RSS alternate element, the comment page does not.
#[cached]
fn head(title: String, add_alternate: bool) -> Node {
    let mut head = Head
        .kid(Meta::name_content("referrer", "origin"))
        .kid(Meta::viewport_sane())
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news.css"))
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news-extra.css"))
        .kid(Link::icon("favicon.ico"));
    if add_alternate {
        head.push_kid(Link::alternate("application/rss+xml", "RSS", "rss"))
    }

    head.kid(Title.text(title))
}

fn table_fatitem(story: &Story) -> Node {
    let score_id = &format!("score_{}", story.id);
    let unv_id = &format!("unv_{}", story.id);
    let item_id = &format!("item?id={}", story.id);
    let time_ago_href = o(A::href(item_id)).add_text(&time_ago(story.submission_time));

    let score_span = Span
        .class("score")
        .id(score_id)
        .text(format!("{} points", story.upvotes));

    let user_href = A::href(&format!("user?id={}", story.author))
        .class("hnuser")
        .text(&story.author);

    let age_span = Span.class("age").set_title("2022-TODO").kid(time_ago_href);

    let unv_span = Span.id(unv_id);
    let hide_a = A::href("TODO").text("hide");
    let past_a = A::href("TODO").class("hnpast").text("past");
    let fav_a = A::href("TODO").text("favorite");
    let comments_a = A::href(item_id).text(format!("{} comments", story.comments.len()));

    let story_url = &story
        .url
        .as_ref()
        .map(|url| url.to_string())
        .unwrap_or_default();

    // Get the short version of the domain in a best effort manner.
    let url_short = story
        .url
        .as_ref()
        .and_then(|url| url.domain().map(|url| url.to_owned()))
        .unwrap_or_else(|| story_url.clone());

    Table
        .class("fatitem")
        .kid(
            Tr.class("athing")
                .id(&story.id.to_string())
                .kid(td().class("title").kid(Span.class("rank")))
                .kid(
                    td().class("votelinks").kid(
                        A::href(&format!(
                            "vote?id={}&amp;how=up&amp;goto=item%3Fid%3D{}",
                            story.id, story.id
                        ))
                        .id(format!("up_{}", story.id))
                        .kid(Div.class("votearrow").set_title("upvote")),
                    ),
                )
                .kid(
                    td().class("title")
                        .kid(A::href(story_url).class("titlelink").text(&story.title))
                        .kid(
                            Span.class("sitebit comhead")
                                .text(" (")
                                .kid(
                                    A::href("from?site=todo")
                                        .kid(Span.class("sitestr").text(&url_short)),
                                )
                                .text(")"),
                        ),
                ),
        )
        .kid(
            Tr.kid(Td::colspan(2)).kid(
                td().class("subtext")
                    .kid(score_span)
                    .text(" by ")
                    .kid(user_href)
                    .text(ONE_SPACE)
                    .kid(age_span)
                    .text(ONE_SPACE)
                    .kid(unv_span)
                    .text(PIPE_DELIMITER)
                    .kid(hide_a)
                    .text(PIPE_DELIMITER)
                    .kid(past_a)
                    .text(PIPE_DELIMITER)
                    .kid(fav_a)
                    .text(PIPE_DELIMITER)
                    .kid(comments_a),
            ),
        )
        .kid(Tr.style("height:10px"))
        .kid(
            Tr.kid(Td::colspan(2)).kid(
                td().kid(
                    Form::new(Method::Post, "comment")
                        .kid(Input::hidden("parent", &story.id.to_string()))
                        .kid(Input::hidden("goto", &format!("item?id={}", story.id)))
                        .kid(Input::hidden("hmac", "hmac-of-what?"))
                        .kid(Textarea::new("text", 8, 80))
                        .kid(Br)
                        .kid(Br)
                        .kid(Input::submit("add comment")),
                ),
            ),
        )
}

fn tr_comment(comment: &Comment) -> Node {
    let id = &comment.id;

    let td_ind = td().class("ind").kid(Img::new_sized("s.gif", 0, 1)); // TODO: Add indent

    let td_votelinks_a = A::href(&format!(
        "vote?id={id}&amp;how=up&amp;goto=item%3Fid%3D{id}"
    ))
    .kid(Div.class("votearrow").set_title("upvote"));
    let td_votelinks = td().class("votelinks").kid(td_votelinks_a);

    /*
    All of this stuff:

    <div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
            <a href="user?id=alecst" class="hnuser">alecst</a> <span class="age"
                title="2022-04-03T14:31:13"><a href="item?id=30897201">6 minutes
                    ago</a></span> <span id="unv_30897201"></span><span
                class="navs"> | <a href="#30897156" class="clicky"
                    aria-hidden="true">next</a></span> <a class="togg clicky"
                id="30897201" n="1" href="javascript:void(0)">[–]</a><span
                class="onstory"></span> </span></div>
    */
    let td_default_div_comhead = Div.kid(
        Span.class("comhead")
            .kid(
                A::href(&format!("user?id={}", comment.author))
                    .class("hnuser")
                    .text(&comment.author),
            )
            .kid(
                Span.class("age")
                    .set_title(&comment.time.to_string())
                    .kid(A::href(&format!("item?id={}", comment.id)).text(time_ago(comment.time))),
            )
            .kid(Span.id(format!("unv_{}", comment.id)))
            .kid(
                Span.class("navs").text(PIPE_DELIMITER).kid(
                    A::href(&format!("#{}", comment.id))
                        .class("clicky")
                        .text("next"),
                ), // TODO: aria-hidden
            )
            .kid(
                A::href("javascript:void(0)")
                    .class("togg clicky")
                    .id(&comment.id.to_string())
                    .text("[-]"), // TODO: n="1", n="<number>", what does it do?
            )
            .kid(Span.class("onstory")),
    );

    /*
    All of this stuff:

    <div class="comment">
        <span class="commtext c00">Actual text goes here
        </span>
        <div class='reply'>
            <p>
                <font size="1">
                    <u><a
                            href="reply?id=30897201&amp;goto=item%3Fid%3D30896661%2330897201">reply</a></u>
                </font>
        </div>
    */
    let td_default_div_comment = Div
        .kid(Span.class("commtext c00").text(&comment.text))
        .kid(Div.class("reply").kid(P.kid(U.kid(A::href("TODO"))))); // TODO: <font> is deprecated, add class.

    let td_default = td()
        .class("default")
        .kid(td_default_div_comhead)
        .kid(Br)
        .kid(td_default_div_comment);

    Tr.class("athing comtr")
        .id(&comment.id.to_string())
        .kid(td().kid(Table.kid(Tr.kid(td_ind).kid(td_votelinks).kid(td_default))))
}

fn table_commen_tree(story: &Story) -> Node {
    let mut table = Table.class("comment-tree");

    for comment in &story.comments {
        table.push_kid(tr_comment(comment));
    }

    table
}

fn body_comments(story: Story) -> Node {
    let fatitem = table_fatitem(&story);
    let comment_tree = table_commen_tree(&story);

    Tr.kid(
        td().kid(fatitem)
            .kid(Br)
            .kid(Br)
            .kid(comment_tree)
            .kid(Br)
            .kid(Br),
    )
}

#[cached]
fn spacer() -> Node {
    Tr.class("spacer").style("height:5px")
}

#[cached]
fn hnstory(story: Story) -> Node {
    let title_link = if let Some(url) = &story.url {
        url.to_string()
    } else {
        format!("item?id={}", story.id)
    };

    let mut title = td()
        .class("title")
        .kid(A::href(&title_link).class("titlelink").text(&story.title));

    if let Some(url) = &story.url {
        let url_long = url.to_string();

        // Get the short version of the domain in a best effort manner.
        let url_short = url
            .domain()
            .map(|short| short.to_string())
            .unwrap_or_else(|| url_long.clone());

        title.push_kid(
            Span.class("sitebit comhead")
                .text(" (")
                .kid(
                    A::href(&format!("from?site={}", &url_short))
                        .kid(Span.class("sitestr"))
                        .text(&url_short),
                )
                .text(")"),
        );
    }

    let comment_text = if story.comments.is_empty() {
        "discuss".into()
    } else {
        format!("{} comments", story.comments.len())
    };

    Node::root()
        .kid(
            Tr.class("athing")
                .id(story.id.to_string())
                .kid(
                    td().class("title-rank")
                        .kid(Span.class("rank").text(format!("{}.", story.rank))),
                )
                .kid(
                    td().class("votelinks").kid(
                        A::href(&format!("vote?id={}&how=up&goto=news", story.id))
                            .id(format!("up_{}", story.id))
                            .kid(Div.class("votearrow").set_title("upvote")),
                    ),
                )
                .kid(title),
        )
        .kid(
            Tr.kid(Td::colspan(2)).kid(
                td().class("subtext")
                    .kid(Span.text(format!("{} points", story.upvotes)))
                    .text(" by ")
                    .kid(
                        A::href(&format!("user?id={}", story.author))
                            .class("hnuser")
                            .text(&story.author)
                            .text(ONE_SPACE),
                    )
                    .kid(
                        Span.class("age")
                            .set_title("2022-03-28T16:35:29") // TODO: This thing
                            .kid(
                                A::href(&format!("item?id={}", story.id))
                                    .text(time_ago(story.submission_time)),
                            ),
                    )
                    .kid(Span)
                    .text(PIPE_DELIMITER)
                    .kid(A::href(&format!("hide?id={}&goto=news", story.id)))
                    .text("hide")
                    .text(PIPE_DELIMITER)
                    .kid(A::href(&format!("item?id={}", { story.id })))
                    .text(&comment_text),
            ),
        )
}

fn body_stories(stories: Vec<Story>) -> Node {
    let mut items = Table.class("itemlist");

    for story in stories {
        items.push_kid(Node::root().kid(hnstory(story)).kid(spacer()))
    }

    Tr.kid(td().kid(items))
}

fn body(body_node: Node) -> Node {
    Body.kid(
        Table
            .id("hnmain")
            .kid(body_nav())
            .kid(body_spacer())
            .kid(body_node)
            .kid(body_footer()),
    )
}

#[cached]
fn script() -> Node {
    Script.into_node()
}

#[cached]
fn body_footer() -> Node {
    let invisible_gif = Img::new_sized("/static/s.gif", 0, 10);
    let divider = Table.kid(Tr.kid(td()).id("footer-divider"));
    let applications = A::href("https://www.ycombinator.com/apply/")
        .text("Applications are open for YC Summer 2022");

    let links = Span
        .class("yclinks")
        .kid(a("newsguidelines.html", "Guidelines"))
        .text(PIPE_DELIMITER)
        .kid(a("newsfaq.html", "FAQ"))
        .text(PIPE_DELIMITER)
        .kid(a("lists", "Lists"))
        .text(PIPE_DELIMITER)
        .kid(a("https://github.com/HackerNews/API", "API"))
        .text(PIPE_DELIMITER)
        .kid(a("security.html", "Security"))
        .text(PIPE_DELIMITER)
        .kid(a("http://www.ycombinator.com/legal/", "Legal"))
        .text(PIPE_DELIMITER)
        .kid(a("http://www.ycombinator.com/apply/", "Apply"))
        .text(PIPE_DELIMITER)
        .kid(a("mailto:hn@ycombinator.com", "Contact"));

    let search = Form::new(form::Method::Get, "//hn.algolia.com/")
        .text("Search: ")
        .kid(Input::text("q", ""));

    Tr.id("footer").kid(
        td().kid(invisible_gif)
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

impl Renderable for Original {
    fn frontpage(&self, stories: Vec<Story>) -> Node {
        for story in stories.iter().take(10) {
            debug!("Id: {} -> {}", story.id, story.title);
        }

        let story_nodes = body_stories(stories);

        o(html::Html)
            .add_attr(("op", "news"))
            .add_attr(Lang::English)
            .kid(head("Hacker News".into(), true))
            .kid(body(story_nodes))
            .kid(script())
    }

    fn comments(&self, story: Story) -> Node {
        let title = format!("{} | Hacker News", story.title);

        let comment_nodes = body_comments(story);

        o(html::Html)
            .add_attr(("op", "item"))
            .add_attr(Lang::English)
            .kid(head(title, false))
            .kid(body(comment_nodes))
            .kid(script())
    }
}
