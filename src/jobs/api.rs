use axum::routing::{delete, post, put};
use axum::{
    Extension, Form,
    extract::Path,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use axum::{Router, extract::Query, routing::get};

use log::error;
use log::warn;

use maud::html;

use super::api;
use crate::model::JobId;
use crate::repository::Limit;
use crate::{jobs::view::card::all, repository::JobRepo};

pub fn router() -> Router {
    Router::new()
        .route("/jobs", get(get_jobs))
        .route("/jobs", put(create))
        .route("/api/jobs/{job_id}", post(api::update_job))
        .route("/api/jobs/{job_id}", delete(api::delete_job))
}

#[axum::debug_handler]
pub async fn get_jobs(repo: Extension<JobRepo>, limit: Query<Limit>) -> impl IntoResponse {
    let limit: Limit = *limit;
    match repo.get_page(limit).await {
        Ok(jobs) => (StatusCode::OK, Ok(html!((all(jobs))))),
        Err(err) => {
            error!("Failed to get a page of jobs: {err}\n{:?}", err.source());
            (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}")))
        }
    }
}

pub async fn create(
    repo: Extension<JobRepo>,
    payload: Form<super::JobRequest>,
) -> impl IntoResponse {
    let Ok(uri) = payload.0.uri.parse::<Uri>() else {
        return (
            StatusCode::BAD_REQUEST,
            Err(format!("{:?} is not a valid URI", payload)),
        );
    };

    let title = payload.title.clone();
    let preface = payload.preface.clone();
    let description = payload.description.clone();

    match repo
        .create(&uri, Some(title), Some(preface), Some(description))
        .await
    {
        // when having admin board, return the created job reference link as html
        Ok(resp) => (StatusCode::OK, Ok(resp.id.to_string())),
        Err(err) => {
            error!(
                "Failed to create job from request: {err}\n{:?}",
                err.source()
            );
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
            warn!(
                "Failed to create job from request: {err}\n{:?}",
                err.source()
            );
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
    payload: Form<super::JobRequest>,
) -> impl IntoResponse {
    let Ok(uri) = payload.uri.parse::<Uri>() else {
        return (
            StatusCode::BAD_REQUEST,
            Err(format!("{:?} is not a valid URI", *payload)),
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
            error!(
                "Failed to create job from request: {err}\n{:?}",
                err.source()
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(format!("Failed to update job with id {}", job_id)),
            )
        }
    }
}
