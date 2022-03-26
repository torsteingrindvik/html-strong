use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

use html_strong::{
    document_tree::{o, Node},
    global_attributes::Lang,
    tags::*,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(hacker_news));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_response(contents: Node) -> Html<String> {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Html(response)
}

fn table() -> Node {
    o(Table).set_id("hnmain")
    // Missing: border (deprecated)
    // Missing: cellpadding (deprecated)
    // Missing: cellspacing (deprecated)
    // Missing: width (deprecated)
    // Missing: bgcolor (deprecated)
}

fn a(href: &str, text: &str) -> Node {
    o(A::href(href)).add_text(text)
}

fn a2(href_text: &str) -> Node {
    o(A::href(href_text)).add_text(href_text)
}

const SPACER: &str = " | ";

fn tr_header() -> Node {
    let td_logo = o(Td).add_style("width:18px;padding-right:4px;").kid(
        o(A::href("https://news.ycombinator.com")).kid(o(Img).add_style("border:10x white solid;")),
    );

    let td_links = o(Td).add_style("line-height:12pt; height:10px;").kid(
        o(Span)
            .add_class("pagetop")
            .kid(o(B).add_class("hnname").kid(a("news", "Hacker News")))
            .kid(a("newest", "new"))
            .add_text(SPACER)
            .kid(a("newcomments", "comments"))
            .add_text(SPACER)
            .kid(a2("ask"))
            .add_text(SPACER)
            .kid(a2("show"))
            .add_text(SPACER)
            .kid(a2("jobs"))
            .add_text(SPACER)
            .kid(a2("submit")),
    );

    let td_login = o(Td).add_style("text-align:right;padding-right:4px;").kid(
        o(Span)
            .add_class("pagetop")
            .kid(a("login?goto=news", "login")),
    );

    o(Tr).kid(
        o(Td).kid(
            o(Table)
                .add_style("padding:2px")
                .kid(o(Tr).kid(td_logo).kid(td_links).kid(td_login)),
        ),
    )
}

fn tr_spacer() -> Node {
    o(Tr)
        .set_id("pagespace")
        .set_title("") // Well ok
        .add_style("height:10px")
}

fn tr_body() -> Node {
    o(Tr)
}

fn tr_footer() -> Node {
    let invisible_gif = o(Img); // Should have height 10, but why?
    let divider = o(Table).kid(o(Tr).kid(Td)); // Should have color
    let applications = o(A::href("https://www.ycombinator.com/apply/"))
        .add_text("Applications are open for YC Summer 2022");

    let links = o(Span)
        .add_class("yclinks")
        .kid(a("newsguidelines.html", "Guidelines"))
        .add_text(SPACER)
        .kid(a("newsfaq.html", "FAQ"))
        .add_text(SPACER)
        .kid(a("lists", "Lists"))
        .add_text(SPACER)
        .kid(a("https://github.com/HackerNews/API", "API"))
        .add_text(SPACER)
        .kid(a("security.html", "Security"))
        .add_text(SPACER)
        .kid(a("http://www.ycombinator.com/legal/", "Legal"))
        .add_text(SPACER)
        .kid(a("http://www.ycombinator.com/apply/", "Apply"))
        .add_text(SPACER)
        .kid(a("mailto:hn@ycombinator.com", "Contact"));

    let search = o(Form);

    o(Tr).kid(
        o(Td)
            .kid(invisible_gif)
            .kid(divider)
            .kid(Br)
            .kid(applications)
            .kid(Br)
            // .kid(links_and_search),
            .kid(links)
            .kid(search),
    )
}

async fn hacker_news() -> Html<String> {
    let head = o(Head)
        .kid(Meta::name_content("referrer", "origin"))
        .kid(Meta::viewport_sane())
        .kid(Link::stylesheet(mime::TEXT_CSS, "news.css"))
        .kid(Link::icon("favicon.ico"))
        .kid(Link::alternate("application/rss+xml", "RSS", "rss"))
        .kid(o(Title).add_text("Hacker News"));

    let table = table()
        .kid(tr_header())
        .kid(tr_spacer())
        .kid(tr_body())
        .kid(tr_footer());

    let body = o(Body).kid(table);

    let script = o(Script);

    let contents = o(html::Html)
        .add_attr(("op", "news"))
        .add_attr(Lang::English)
        .kid(head)
        .kid(body)
        .kid(script);

    get_response(contents)
}
