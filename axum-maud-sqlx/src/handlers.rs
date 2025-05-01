use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use tracing_log::log::{info, warn};

use crate::{
    model::{Job, JobId, Limit},
    repository::JobRepo,
};
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
                            a href = (format!("/jobs/{}", job.id))  {"Job: " (job.id) " – "}
                            a href = (job.uri) {(job.uri)}
                        }
                    }
                }
            }),
        ),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}"))),
    }
}

pub async fn get_job(repo: Extension<JobRepo>, job_id: Path<JobId>) -> impl IntoResponse {
    info!("Got job {}?", *job_id);
    match repo.get_one(*job_id).await {
        Ok(Some(Job { id, uri })) => (
            StatusCode::OK,
            Ok(html! {
                p {
                    a href = "/jobs" {"Back"}
                }
                p {
                    "Job: " (id) " – "
                    a href = (uri) {(uri)}
                }
            }),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Err(format!("Job with id {} not found", *job_id)),
        ),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}"))),
    }
}

pub async fn create_job(repo: Extension<JobRepo>, uri: Json<String>) -> impl IntoResponse {
    let Ok(uri) = uri.parse::<Uri>() else {
        return (
            StatusCode::BAD_REQUEST,
            Err(format!("{} is not a valid URI", *uri)),
        );
    };

    match repo.create(&uri).await {
        Ok(_) => (StatusCode::OK, Ok("Ok")),
        Err(err) => {
            warn!("Failed to create job from request: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err("Failed to create a job".to_string()),
            )
        }
    }
}

pub async fn delete_job(repo: Extension<JobRepo>, job_id: Path<JobId>) -> impl IntoResponse {
    let job_id: JobId = *job_id;
    match repo.delete(job_id).await {
        Ok(Some(_)) => (StatusCode::OK, Ok("Ok")),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Err(format!("Tried to update non existant job {job_id}")),
        ),
        Err(err) => {
            warn!("Failed to create job from request: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(format!("Failed to update job with id {}", job_id)),
            )
        }
    }
}
pub async fn update_job(
    repo: Extension<JobRepo>,
    job_id: Path<JobId>,
    uri: Json<String>,
) -> impl IntoResponse {
    let Ok(uri) = uri.parse::<Uri>() else {
        return (
            StatusCode::BAD_REQUEST,
            Err(format!("{} is not a valid URI", *uri)),
        );
    };
    let uri = uri.into();

    let job_id: JobId = *job_id;
    match repo.update(job_id, &uri).await {
        Ok(Some(_)) => (StatusCode::OK, Ok("Ok")),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Err(format!("Tried to update non existant job {job_id}")),
        ),
        Err(err) => {
            warn!("Failed to create job from request: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(format!("Failed to update job with id {}", job_id)),
            )
        }
    }
}
