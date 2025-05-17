use axum::{
    Extension, Form,
    extract::Path,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::{model::JobId, repository::JobRepo};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobRequest {
    uri: String,
    title: String,
    preface: String,
    description: String,
}

// todo move to jobs module
pub async fn create_job(repo: Extension<JobRepo>, payload: Form<JobRequest>) -> impl IntoResponse {
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
        // todo return json
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
    payload: Form<JobRequest>,
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
