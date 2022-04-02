use axum::{
    extract::Extension,
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Router,
};
use std::{io, net::SocketAddr, sync::Arc, time::Instant};
use tower::ServiceBuilder;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;

use html_strong::{
    document_tree::{o, Node},
    global_attributes::Lang,
    tags::html,
};
use tokio::sync::RwLock;

// type SharedState = Arc<RwLock<hacker_news::State>>;

fn get_response(contents: Node) -> Html<String> {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Html(response)
}

#[derive(Debug, Default, Clone)]
pub struct SharedState(pub Arc<RwLock<Vec<hacker_news::post::Post>>>);

mod hacker_news {

    use cached::proc_macro::cached;
    use html_strong::{
        document_tree::{o, Node},
        tags::td::td,
        tags::*,
    };

    const PIPE_DELIMITER: &str = " | ";

    // Shows in inspector as a "whitespace only text node" (in some browsers)
    const ONE_SPACE: &str = " ";

    fn a(href: &str, text: &str) -> Node {
        o(A::href(href)).add_text(text)
    }

    fn a2(href_text: &str) -> Node {
        o(A::href(href_text)).add_text(href_text)
    }

    #[cached]
    fn body_nav() -> Node {
        let td_logo = o(td()).add_style("width:18px;padding-right:4px;").kid(
            o(A::href("https://news.ycombinator.com")).kid(
                o(Img::new_sized("/static/y18.gif", 18, 18)).add_style("border:1px white solid;"),
            ),
        );

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
    pub fn head() -> Node {
        o(Head)
            .kid(Meta::name_content("referrer", "origin"))
            .kid(Meta::viewport_sane())
            .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news.css"))
            .kid(Link::stylesheet(mime::TEXT_CSS, "/static/news-extra.css"))
            .kid(Link::icon("favicon.ico"))
            .kid(Link::alternate("application/rss+xml", "RSS", "rss"))
            .kid(o(Title).add_text("Hacker News"))
    }

    pub mod post {
        use super::*;
        use crate::SharedState;

        use anyhow::Result;
        use futures::{future, stream::FuturesOrdered, FutureExt, StreamExt};
        use serde::Deserialize;
        use std::time::{Duration, Instant};
        use tracing::{info, trace, warn};

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
        pub struct Post {
            /// Author
            pub by: String,
            /// Number of comments
            pub descendants: usize,
            /// Story id
            pub id: usize,
            /// Comments
            // pub kids: usize,
            /// Upvotes
            pub score: usize,
            /// Time in ?? format
            pub time: usize,
            /// Story title
            pub title: String,
            /// Story url. TODO: Short version?
            pub url: Option<String>,

            /// Not in JSON, will be set by us
            pub rank: Option<usize>,
        }

        async fn get_new_story_ids() -> Result<Vec<usize>> {
            Ok(
                reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
                    .await?
                    .json()
                    .await?,
            )
        }

        async fn get_story(id: usize) -> Result<post::Post> {
            let story_raw = reqwest::get(format!(
                "https://hacker-news.firebaseio.com/v0/item/{id}.json"
            ))
            .await?;
            trace!("Story raw: {story_raw:#?}");

            Ok(story_raw.json().await?)
        }

        async fn get_posts() -> Result<Vec<post::Post>> {
            let mut story_ids = get_new_story_ids().await?;
            info!("Pulled {} story ids", story_ids.len());
            story_ids.truncate(50);

            let mut futures = FuturesOrdered::new();

            for id in story_ids {
                futures.push(get_story(id).map(move |fut| (fut, id)));
            }

            let now = Instant::now();

            let posts = futures
                .inspect(|(fut, id)| {
                    if let Err(e) = fut {
                        warn!("Problem (id: {id}) getting post: {e:?}");
                    }
                })
                .filter_map(|(fut, _)| future::ready(fut.ok()))
                .collect::<Vec<_>>()
                .await;

            info!("Posts resolved in {:?}", now.elapsed());

            Ok(posts)
        }

        pub async fn worker(state: SharedState) {
            info!("API worker started");

            loop {
                let posts = match get_posts().await {
                    Ok(posts) => posts,
                    Err(e) => {
                        warn!("Couldn't fetch stories: {e:?}");
                        continue;
                    }
                };

                {
                    let now = Instant::now();
                    let mut state = state.0.write().await;
                    *state = posts;
                    info!("New post added (held write lock for {:?})", now.elapsed());
                }

                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        }

        #[cached]
        fn spacer() -> Node {
            o(Tr).add_class("spacer").add_style("height:5px")
        }

        #[cached]
        fn post(post: Post) -> Node {
            let link_thing: Node = if let Some(url) = post.url {
                o(td())
                    .add_class("title")
                    .kid(
                        o(A::href(&url))
                            .add_class("titlelink")
                            .add_text(&post.title),
                    )
                    .kid(
                        o(Span)
                            .add_class("sitebit comhead")
                            .add_text(" (")
                            .kid(
                                o(A::href(&format!("from?site={}", &url)))
                                    .kid(o(Span).add_class("sitestr"))
                                    .add_text(&url),
                            )
                            .add_text(")"),
                    )
            } else {
                Node::root()
            };

            let comment_text = if post.descendants == 0 {
                "discuss".into()
            } else {
                format!("{} comments", post.descendants)
            };

            Node::root()
                .kid(
                    o(Tr)
                        .add_class("athing")
                        .set_id(&post.id.to_string())
                        .kid(
                            o(td()).add_class("title-rank").kid(
                                o(Span)
                                    .add_class("rank")
                                    .add_text(&format!("{}.", post.rank.expect("Must set rank"))),
                            ),
                        )
                        .kid(
                            o(td()).add_class("votelinks").kid(
                                o(A::href(&format!("vote?id={}&how=up&goto=news", post.id)))
                                    .set_id(&format!("up_{}", post.id))
                                    .kid(o(Div).add_class("votearrow").set_title("upvote")),
                            ),
                        )
                        .kid(link_thing),
                )
                .kid(
                    o(Tr).kid(Td::colspan(2)).kid(
                        o(td())
                            .add_class("subtext")
                            .kid(o(Span).add_text(&format!("{} points", post.score)))
                            .add_text(" by ")
                            .kid(
                                o(A::href(&format!("user?id={}", post.by)))
                                    .add_class("hnuser")
                                    .add_text(&post.by)
                                    .add_text(ONE_SPACE),
                            )
                            .kid(
                                o(Span)
                                    .add_class("age")
                                    .set_title("2022-03-28T16:35:29") // TODO: This thing
                                    .kid(
                                        o(A::href(&format!("item?id={}", post.id)))
                                            .add_text("1 hour ago"),
                                    ),
                            )
                            .kid(Span)
                            .add_text(PIPE_DELIMITER)
                            .kid(
                                o(A::href(&format!("hide?id={}&goto=news", post.id)))
                                    .add_text("hide"),
                            )
                            .add_text(PIPE_DELIMITER)
                            .kid(
                                o(A::href(&format!("item?id={}", { post.id })))
                                    .add_text(&comment_text),
                            ),
                    ),
                )
        }

        #[allow(clippy::from_over_into)]
        impl Into<Node> for Post {
            fn into(self) -> Node {
                Node::root().kid(post(self)).kid(spacer())
            }
        }
    }

    fn body_posts(posts: Vec<post::Post>) -> Node {
        let mut items = o(Table).add_class("itemlist");

        for (rank, mut post) in posts.into_iter().enumerate() {
            post.rank = Some(rank + 1);
            items.push_kid(post)
        }

        o(Tr).kid(o(td()).kid(items))
    }

    #[cached]
    pub fn body(posts: Vec<post::Post>) -> Node {
        o(Body).kid(
            o(Table)
                .set_id("hnmain")
                .kid(body_nav())
                .kid(body_spacer())
                .kid(body_posts(posts))
                .kid(body_footer()),
        )
    }

    #[cached]
    pub fn script() -> Node {
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
}

async fn hacker_news(Extension(state): Extension<SharedState>) -> Html<String> {
    let now = Instant::now();
    let posts = state.0.read().await.clone();
    info!("Post acquired (held read lock for {:?})", now.elapsed());

    get_response(
        o(html::Html)
            .add_attr(("op", "news"))
            .add_attr(Lang::English)
            .kid(hacker_news::head())
            .kid(hacker_news::body(posts))
            .kid(hacker_news::script()),
    )
}

async fn internal_server_error(error: io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = SharedState::default();
    tokio::spawn(hacker_news::post::worker(state.clone()));

    // build our application with a route
    let app = Router::new()
        .route("/", get(hacker_news))
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("examples-static/favicon.ico"))
                .handle_error(internal_server_error),
        )
        .nest(
            "/static",
            get_service(ServeDir::new("examples-static")).handle_error(internal_server_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(state)),
        );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
