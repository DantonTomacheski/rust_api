use tokio::net::TcpListener;
use tracing::{Level, info};

mod error2;
mod web2;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!(%addr, "Listening on:");

    axum::serve(listener, web2::routes_login2::routes()).await.unwrap();
}