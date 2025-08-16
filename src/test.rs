use crate::{
    config::{self, Testing},
    scraper::{self},
    show_jobs,
};

pub async fn test_concurrent_jobs() {
    println!("Testing concurrent job processing...\n");

    let test_config = config::Config::test_instance();

    let job_postings = scraper::scrape_jobs_postings(&test_config).await;

    show_jobs::render_result(job_postings);
}
