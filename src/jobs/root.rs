use axum::{Router, routing::get};

use crate::http_types::{OxHtml, ox_html};

use super::job_card;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .merge(job_card::router())
}

async fn index() -> OxHtml {
    ox_html(super::ui().into_string())
}
