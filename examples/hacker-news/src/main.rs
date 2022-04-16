use hacker_news::server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    server::run([127, 0, 0, 1], 3000).await;
}
