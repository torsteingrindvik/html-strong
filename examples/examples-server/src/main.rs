use axum::{routing::get_service, Router};
use examples_lib::Example;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, services::ServeFile, trace::TraceLayer};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let home = examples_lib::Home;
    let blog = markdown_blog::server::MarkdownBlog;
    let hn = hacker_news::server::HackerNews::new();

    let app = Router::new()
        .nest("/", home.router("../examples-lib"))
        .nest("/blog", blog.router("../markdown-blog"))
        .nest("/hn", hn.router("../hacker-news"))
        // TODO: Shared favicon
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("static/favicon.ico"))
                .handle_error(examples_lib::internal_server_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new()),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
