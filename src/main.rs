use axum::{
    Router,
    extract::{Path, Request},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::{Level, error, info};

mod error;
mod web;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback(handler_404);

    // region: Start server
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Server is enabled at: {:?}", addr);

    axum::serve(listener, routes_all).await.unwrap();
    // endregion: Start server
}

async fn main_response_mapper(res: Response) -> Response {
    info!("{:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

async fn handler_404(req: Request) -> Response {
    error!("Route not found: {} {}", req.method(), req.uri());
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

// region: Routes Hello
fn routes_hello() -> Router {
    Router::new().route("/hello/{name}", get(handler_hello))
}

async fn handler_hello(Path(name): Path<String>) -> impl IntoResponse {
    info!("{:<12} - handler_hello", "HANDLER");

    Html(format!("Name is: {:?}", name))
}
// endregion: Routes Hello
