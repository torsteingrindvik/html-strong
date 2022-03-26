use axum::{response::Html, routing::get, Router};
use std::{net::SocketAddr, path::PathBuf};

use html_strong::{
    document_tree::{o, Node},
    global_attributes::Id,
    tags::*,
    template,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/tree", get(handler_tree))
        .route("/disk", get(handler_from_disk));

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

async fn handler() -> Html<String> {
    let contents = o(h1::H1).add_text("Hello, World! :-)");

    get_response(contents)
}

async fn handler_from_disk() -> Html<String> {
    // Read some text from disk.
    let path = PathBuf::from(format!(
        "{}/examples/text_input.txt",
        std::env!("CARGO_MANIFEST_DIR"),
    ));
    let text = tokio::fs::read_to_string(path)
        .await
        .expect("Should be able to read file");

    // Create a response from it.
    let body = o(h1::H1).add_text(&text);
    get_response(
        template::HtmlDocumentBuilder::new()
            .with_head(template::head().kid(Meta::refresh(1))) // Auto-refresh, try changing it on disk!
            .with_body(body)
            .build(),
    )
}

async fn handler_tree() -> Html<String> {
    let body = o(Body)
        .kid(
            o(Div)
                .add_attr(Id::new("content"))
                .kid(o(H1).add_text("Heading here"))
                .kid(o(P).add_text("Lorem ipsum dolor sit amet."))
                .kid(
                    o(P).add_text("Lorem ipsum dolor ")
                        .kid(o(Em).add_text("sit"))
                        .add_text(" amet."),
                )
                .kid(Hr),
        )
        .kid(
            o(Div).add_attr(Id::new("nav")).kid(
                o(Ul)
                    .kid(o(Li).add_text("item 1"))
                    .kid(o(Li).add_text("item 2"))
                    .kid(o(Li).add_text("item 3"))
                    .kid(o(Li).add_text("item 4"))
                    .kid(o(Li).add_text("item 5")),
            ),
        );

    get_response(template::HtmlDocumentBuilder::new().with_body(body).build())
}
