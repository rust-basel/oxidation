use std::time::Duration;

use tokio::time::sleep;

use crate::{
    config::{Config, JobSource},
    job_processor, scraper,
};

pub struct JobPosting {
    pub title: String,
    pub company: String,
    pub location: String,
    pub language: String,
    pub programming_language: String,
    pub description: String,
    pub url: String,
}

pub async fn scrape_jobs_postings(config: &Config) -> Vec<JobPosting> {
    let static_job_postings = Vec::new();

    let async_jobs: Vec<
        std::pin::Pin<Box<dyn std::future::Future<Output = Vec<scraper::JobPosting>> + Send>>,
    > = create_jobs(config);

    let start = std::time::Instant::now();
    let results = job_processor::process_jobs_concurrent(static_job_postings, async_jobs).await;
    let duration = start.elapsed();

    println!(
        "Processed {} jobs concurrently in {:?}\n",
        results.len(),
        duration
    );
    results
}

fn create_jobs(
    config: &Config,
) -> Vec<std::pin::Pin<Box<dyn std::future::Future<Output = Vec<scraper::JobPosting>> + Send + '_>>>
{
    let mut jobs: Vec<
        std::pin::Pin<Box<dyn std::future::Future<Output = Vec<scraper::JobPosting>> + Send>>,
    > = Vec::new();
    config
        .job_sources
        .iter()
        .for_each(|source| jobs.push(Box::pin(create_simple_job(source))));
    jobs
}

async fn create_simple_job(source: &JobSource) -> Vec<scraper::JobPosting> {
    if let Some(work) = source.wait {
        simulate_random_work(work).await;
    }

    //todo implement the actual scraping from the url

    vec![]
}

async fn simulate_random_work(work: u64) {
    sleep(Duration::from_millis(work)).await;
}
