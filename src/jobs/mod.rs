mod api;
mod root;
mod view;

use axum::{Router, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobRequest {
    uri: String,
    title: String,
    preface: String,
    description: String,
}

pub fn router() -> Router {
    Router::new().route("/", get(index)).merge(api::router())
}

async fn index() -> impl IntoResponse {
    view::index()
}
