use axum::{response::Html, routing::get, Router};
use std::{net::SocketAddr, path::PathBuf};

use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*, template};

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
    get_response(H1.text("Hello, World! :-)"))
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
    let body = H1.text(&text);
    get_response(
        template::HtmlDocumentBuilder::new()
            .with_head(template::head().kid(Meta::refresh(1))) // Auto-refresh, try changing it on disk!
            .with_body(body)
            .build(),
    )
}

async fn handler_tree() -> Html<String> {
    let body = Body
        .kid(
            Div.id("content")
                .kid(H1.text("Heading here"))
                .kid(P.text("Lorem ipsum dolor sit amet."))
                .kid(
                    P.text("Lorem ipsum dolor ")
                        .kid(Em.text("sit"))
                        .text(" amet."),
                )
                .kid(Hr),
        )
        .kid(
            Div.id("nav").kid(
                Ul.kid(Li.text("item 1"))
                    .kid(Li.text("item 2"))
                    .kid(Li.text("item 3"))
                    .kid(Li.text("item 4"))
                    .kid(Li.text("item 5")),
            ),
        );

    get_response(template::HtmlDocumentBuilder::new().with_body(body).build())
}
