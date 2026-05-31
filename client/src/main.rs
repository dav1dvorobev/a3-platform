mod routes;
mod state;

use axum::{
    Router,
    routing::{any, get},
};
use routes::{health, ws};
use state::AppState;
use tower_http::services::ServeDir;

const STATIC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

#[tokio::main]
async fn main() {
    let port = std::env::var("CLIENT_PORT").unwrap();
    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .unwrap();
    let router = Router::new()
        .route("/api/ws", any(ws))
        .route("/api/health", get(health))
        .fallback_service(ServeDir::new(STATIC_DIR).append_index_html_on_directories(true))
        .with_state(AppState::new());
    axum::serve(listener, router).await.unwrap();
}
