use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/healthz", get(health))
}

async fn health() {}
