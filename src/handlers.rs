use axum::{
    Extension, Json,
    extract::Query,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use tracing_log::log::warn;

use crate::{model::Limit, repository::JobRepo};
use maud::html;

#[axum::debug_handler]
pub async fn get_jobs(repo: Extension<JobRepo>, limit: Query<Limit>) -> impl IntoResponse {
    let limit: Limit = *limit;
    match repo.get_page(limit).await {
        Ok(jobs) => (
            StatusCode::OK,
            Ok(html! {
                p {
                    @for job in &jobs {
                        li {
                            "Job: " (job.id) " – "
                            a href = (job.uri) {(job.uri)}
                        }
                    }
                }
            }),
        ),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}"))),
    }
}

#[axum::debug_handler]
pub async fn get_jobs(repo: Extension<JobRepo>, limit: Query<Limit>) -> impl IntoResponse {
    let limit: Limit = *limit;
    match repo.get_page(limit).await {
        Ok(jobs) => (
            StatusCode::OK,
            Ok(html! {
                p {
                    @for job in &jobs {
                        li {
                            "Job: " (job.id) " – "
                            a href = (job.uri) {(job.uri)}
                        }
                    }
                }
            }),
        ),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}"))),
    }
}

pub async fn create_job(repo: Extension<JobRepo>, uri: Json<String>) -> impl IntoResponse {
    let Ok(uri) = uri.parse::<Uri>() else {
        return (StatusCode::BAD_REQUEST, Err("Expected Uri"));
    };

    match repo.create(&uri).await {
        Ok(_) => (StatusCode::OK, Ok("Ok")),
        Err(err) => {
            warn!("Failed to create job from request: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, Err("Failed to create "))
        }
    }
}
