use crate::scraper::JobPosting;
use std::future::Future;

#[allow(unused)]
pub async fn process_jobs<F>(mut job_postings: Vec<JobPosting>, jobs: Vec<F>) -> Vec<JobPosting>
where
    F: Future<Output = JobPosting>,
{
    for job in jobs {
        let posting = job.await;
        job_postings.push(posting);
    }
    job_postings
}

pub async fn process_jobs_concurrent<F>(
    mut job_postings: Vec<JobPosting>,
    jobs: Vec<F>,
) -> Vec<JobPosting>
where
    F: Future<Output = Vec<JobPosting>>,
{
    let results = futures::future::join_all(jobs).await;
    let results = results.into_iter().flatten().collect::<Vec<_>>();
    job_postings.extend(results);
    job_postings
}
