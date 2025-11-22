use tokio::net::TcpListener;
use tracing::{Level, info};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Listening on addr {:?}", listener.local_addr().unwrap());

    axum::serve(listener, web::routes_login::routes()).await.unwrap();
}