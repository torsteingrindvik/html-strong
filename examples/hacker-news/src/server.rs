use crate::{
    frontend::{self, Frontend, Renderable},
    settings::Settings,
    state::SharedState,
};
use axum::{
    extract::{Extension, Form, Query},
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use html_strong::document_tree::Node;
use reqwest::StatusCode;
use serde::Deserialize;
use std::{borrow::Cow, convert::Infallible, net::SocketAddr, time::Instant};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::debug;

type Result = std::result::Result<(CookieJar, Html<String>), Cow<'static, str>>;

fn get_response(contents: Node, jar: CookieJar) -> Result {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Ok((jar, Html(response)))
}

fn make_cookie(frontend: &Frontend) -> Cookie<'static> {
    let mut cookie = Cookie::new(frontend::Frontend::COOKIE_NAME, frontend.to_string());
    cookie.make_permanent();
    cookie
}

fn get_frontend(jar: CookieJar) -> (frontend::Frontend, CookieJar) {
    if let Some(choice) = jar.get(frontend::Frontend::COOKIE_NAME).and_then(|cookie| {
        let to_choice = cookie.value().try_into();
        debug!("Result of turning cooke into choice: {to_choice:?}");
        to_choice.ok()
    }) {
        debug!("User had stored frontend choice: {choice}");
        (choice, jar)
    } else {
        let frontend = frontend::Frontend::default();
        let jar = jar.add(make_cookie(&frontend));
        debug!("User had no stored frontend choice, setting cookie with default: {frontend}");
        (frontend, jar)
    }
}

async fn front_page(Extension(state): Extension<SharedState>, jar: CookieJar) -> Result {
    let now = Instant::now();
    let stories = state.0.read().await.clone();

    debug!("Stories acquired (held read lock for {:?})", now.elapsed());

    for story in stories.iter().take(10) {
        debug!("Id: {} -> {}", story.id, story.title);
    }

    let (frontend_choice, jar) = get_frontend(jar);
    get_response(frontend_choice.frontpage(stories), jar)
}

#[derive(Debug, Deserialize)]
pub struct Item {
    id: usize,
}

async fn comment_page(
    Query(Item { id }): Query<Item>,
    Extension(state): Extension<SharedState>,
    jar: CookieJar,
) -> Result {
    if let Some(story) = state
        .0
        .read()
        .await
        .iter()
        .find(|story| story.id == id)
        .cloned()
    {
        let (frontend_choice, jar) = get_frontend(jar);
        get_response(frontend_choice.comments(story), jar)
    } else {
        Err(format!("No such story: {id}").into())
    }
}

async fn settings_page(jar: CookieJar) -> Result {
    let (current_choice, jar) = get_frontend(jar);
    let settings = Settings::new_with_options("choice", Frontend::as_options(current_choice));

    get_response(settings.into_page(), jar)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    choice: String,
}

async fn settings_post(jar: CookieJar, Form(Input { choice }): Form<Input>) -> impl IntoResponse {
    let jar = if let Ok(frontend) = choice.as_str().try_into() {
        let cookie = make_cookie(&frontend);
        jar.add(cookie)
    } else {
        jar
    };

    std::result::Result::<_, Infallible>::Ok((jar, Redirect::to("/")))
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
        .route("/settings", get(settings_page).post(settings_post))
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
