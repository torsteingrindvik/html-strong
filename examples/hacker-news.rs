use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

use html_strong::{
    document_tree::{o, Node},
    global_attributes::Lang,
    tags::td::td,
    tags::*,
};

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

fn tr_nav() -> Node {
    let td_logo = o(td())
        .add_style("width:18px;padding-right:4px;")
        .kid(o(A::href("https://news.ycombinator.com")).kid(
            o(Img::new_sized("/static/y18.gif", 18, 18)).add_style("border:1px white solid;"),
        ));

    let td_links = o(td()).add_style("line-height:12pt; height:10px;").kid(
        o(Span)
            .add_class("pagetop")
            .kid(o(B).add_class("hnname").kid(a("news", "Hacker News")))
            .add_text(" ") // Shows in inspector as a "whitespace only text node" (in some browsers)
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

fn tr_spacer() -> Node {
    o(Tr)
        .set_id("pagespace")
        .set_title("") // Well ok
        .add_style("height:10px")
}

fn entry_main() -> Node {
    o(Tr)
        .add_class("athing")
        .set_id("12345678")
        .kid(
            o(td()) // Missing: align, valign
                .add_class("title")
                .kid(o(Span).add_class("rank").add_text("1.")),
        )
        .kid(
            o(td()).add_class("votelinks").kid(
                o(A::href("vote?id=12345678&how=up&goto=news"))
                    .kid(o(Div).add_class("votearrow").set_title("upvote")),
            ),
        )
        .kid(
            o(td())
                .add_class("title")
                .kid(
                    o(A::href("https://nrk.no"))
                        .add_class("titlelink")
                        .add_text("Thing happens and it makes the news"),
                )
                .kid(
                    o(Span)
                        .add_class("sitebit")
                        .add_class("comhead")
                        .add_text(" (")
                        .kid(
                            o(A::href("from?site=nrk.no"))
                                .kid(o(Span).add_class("sitestr"))
                                .add_text("nrk.no"),
                        )
                        .add_text(")"),
                ),
        )
}

fn entry_points() -> Node {
    o(Tr).kid(Td::colspan(2)).kid(
        o(td())
            .add_class("subtext")
            .kid(o(Span).add_text("123 points"))
            .add_text(" by ")
            .kid(
                o(A::href("user?id=togr"))
                    .add_class("hnuser")
                    .add_text("togr"),
            )
            .kid(
                o(Span)
                    .add_class("age")
                    .set_title("2022-03-28T16:35:29")
                    .kid(o(A::href("item?id=12345678")).add_text("1 hour ago")),
            )
            .kid(Span)
            .add_text(SPACER)
            .kid(o(A::href("hide?id=12345678&goto=news")).add_text("hide"))
            .add_text(SPACER)
            .kid(o(A::href("item?id=12345678")).add_text("130 comments")),
    )
}

fn entry_spacer() -> Node {
    o(Tr).add_class("spacer").add_style("height:5px")
}

fn tr_body() -> Node {
    let mut items = o(Table).add_class("itemlist");

    for _ in 0..25 {
        items.push_kid(entry_main());
        items.push_kid(entry_points());
        items.push_kid(entry_spacer());
    }

    o(Tr).kid(o(td()).kid(items))
}

fn tr_footer() -> Node {
    let invisible_gif = o(Img::new_sized("/static/s.gif", 0, 10));
    let divider = o(Table).kid(o(Tr).kid(td()).set_id("footer-divider"));
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

async fn hacker_news() -> Html<String> {
    let head = o(Head)
        .kid(Meta::name_content("referrer", "origin"))
        .kid(Meta::viewport_sane())
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news.css"))
        .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news-extra.css"))
        .kid(Link::icon("favicon.ico"))
        .kid(Link::alternate("application/rss+xml", "RSS", "rss"))
        .kid(o(Title).add_text("Hacker News"));

    let table = table()
        .kid(tr_nav())
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(hacker_news))
        .nest(
            "/static",
            get_service(ServeDir::new("examples-static")).handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .layer(TraceLayer::new_for_http());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
