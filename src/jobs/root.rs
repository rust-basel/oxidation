use axum::{Router, routing::get};

use crate::http_types::{OxHtml, ox_html};

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> OxHtml {
    ox_html(super::ui().into_string())
}
