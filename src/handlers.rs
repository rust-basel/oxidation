use axum::{
    Extension, Json,
    extract::Path,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use log::{error, warn};

use crate::{model::JobId, repository::JobRepo};

pub async fn create_job(repo: Extension<JobRepo>, uri: Json<String>) -> impl IntoResponse {
    let Ok(uri) = uri.parse::<Uri>() else {
        return (
            StatusCode::BAD_REQUEST,
            Err(format!("{} is not a valid URI", *uri)),
        );
    };

    match repo.create(&uri).await {
        Ok(resp) => (StatusCode::OK, Ok(Json(resp))),
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
