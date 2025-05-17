use axum::{
    Extension, Router,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use log::{error, info};
use maud::{Markup, html};

use crate::{
    jobs::job_card,
    model::{Job, JobId, Limit},
    repository::JobRepo,
};
pub fn router() -> Router {
    Router::new()
        .route("/jobs", get(get_jobs))
        .route("/jobs/{job_id}", get(get_job))
}

#[axum::debug_handler]
pub async fn get_jobs(repo: Extension<JobRepo>, limit: Query<Limit>) -> impl IntoResponse {
    let limit: Limit = *limit;
    match repo.get_page(limit).await {
        Ok(jobs) => (
            StatusCode::OK,
            // todo return new view
            Ok(html!((jobs_view(jobs)))),
        ),
        Err(err) => {
            error!("Failed to get a page of jobs: {err}\n{:?}", err.source());
            (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}")))
        }
    }
}

pub fn jobs_view(jobs: Vec<Job>) -> Markup {
    html! {
        @for job in jobs{
            (job_card(job.title(), job.preface(), vec![html!{
                div class="badge badge-accent" {
                    "On-site"
                }
            }], job.description(), html!{
                a class="btn btn-secondary btn-sm bg-base" href=(job.uri()) {
                "Apply"
            }}))
        }
    }
}

pub async fn get_job(repo: Extension<JobRepo>, job_id: Path<JobId>) -> impl IntoResponse {
    info!("Got job {}?", *job_id);
    match repo.get_one(*job_id).await {
        Ok(Some(_job)) => (
            StatusCode::OK,
            // todo return new view
            Ok(""),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Err(format!("Job with id {} not found", *job_id)),
        ),
        Err(err) => {
            error!("Failed to get job {}. {err}\n{:?}", *job_id, err.source());
            (StatusCode::INTERNAL_SERVER_ERROR, Err(format!("{err}")))
        }
    }
}
