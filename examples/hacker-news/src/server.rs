use crate::frontend::{self, Renderable};
use crate::state::SharedState;
use axum::extract::Query;
use axum::response::Html;
use axum::{
    extract::Extension,
    routing::{get, get_service},
    Router,
};
use html_strong::document_tree::Node;
use reqwest::StatusCode;
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Instant;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::debug;

fn get_response(contents: Node) -> Html<String> {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Html(response)
}

async fn front_page(Extension(state): Extension<SharedState>) -> Html<String> {
    let now = Instant::now();
    let stories = state.0.read().await.clone();

    debug!("Stories acquired (held read lock for {:?})", now.elapsed());

    for story in stories.iter().take(10) {
        debug!("Id: {} -> {}", story.id, story.title);
    }

    let choice = frontend::Frontend::default();

    get_response(choice.frontpage(stories))
}

#[derive(Debug, Deserialize)]
pub struct Item {
    id: usize,
}

async fn comment_page(
    Query(Item { id }): Query<Item>,
    Extension(state): Extension<SharedState>,
) -> Html<String> {
    if let Some(story) = state
        .0
        .read()
        .await
        .iter()
        .find(|story| story.id == id)
        .cloned()
    {
        let choice = frontend::Frontend::default();
        get_response(choice.comments(story))
    } else {
        format!("TODO error handle, missing id {id:?}").into()
    }
}

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
        .route("/", get(front_page))
        .route("/item", get(comment_page))
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
