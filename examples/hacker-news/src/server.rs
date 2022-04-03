use crate::html::hacker_news;
use crate::state::SharedState;
use axum::{
    extract::Extension,
    routing::{get, get_service},
    Router,
};
use reqwest::StatusCode;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

async fn internal_server_error(error: std::io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}

pub async fn run(ip: [u8; 4], port: u16) {
    let state = SharedState::default();
    tokio::spawn(crate::state_worker::worker(state.clone()));

    // build our application with a route
    let app = Router::new()
        .route("/", get(hacker_news))
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("static/favicon.ico")).handle_error(internal_server_error),
        )
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(internal_server_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(state))
                .layer(CompressionLayer::new()),
        );

    // run it
    let addr = SocketAddr::from((ip, port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
