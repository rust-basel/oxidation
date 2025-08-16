use std::time::Duration;

use tokio::time::sleep;

use crate::{
    job_processor,
    scraper::{self, JobPosting},
    show_jobs,
};

pub async fn test_concurrent_jobs() {
    println!("Testing concurrent job processing...\n");

    let static_job_postings = Vec::new();

    // Box the futures to make them the same type
    let async_jobs: Vec<
        std::pin::Pin<Box<dyn std::future::Future<Output = scraper::JobPosting> + Send>>,
    > = vec![
        Box::pin(create_rust_job()),
        Box::pin(create_go_job()),
        Box::pin(create_typescript_job()),
    ];

    let start = std::time::Instant::now();
    let results = job_processor::process_jobs_concurrent(static_job_postings, async_jobs).await;
    let duration = start.elapsed();

    println!(
        "Processed {} jobs concurrently in {:?}\n",
        results.len(),
        duration
    );
    show_jobs::render_result(results);
}

async fn create_rust_job() -> JobPosting {
    simulate_random_work().await;
    JobPosting {
        title: "Rust Developer".to_string(),
        company: "Mozilla".to_string(),
        location: "Remote".to_string(),
        language: "English".to_string(),
        programming_language: "Rust".to_string(),
        description: "Build systems programming applications with Rust".to_string(),
        url: "https://mozilla.org/careers".to_string(),
    }
}

async fn create_go_job() -> JobPosting {
    simulate_random_work().await;
    JobPosting {
        title: "Go Backend Engineer".to_string(),
        company: "Docker".to_string(),
        location: "San Francisco, CA".to_string(),
        language: "English".to_string(),
        programming_language: "Go".to_string(),
        description: "Develop containerization and orchestration tools".to_string(),
        url: "https://docker.com/careers".to_string(),
    }
}

async fn create_typescript_job() -> JobPosting {
    simulate_random_work().await;
    JobPosting {
        title: "TypeScript Full Stack Developer".to_string(),
        company: "Vercel".to_string(),
        location: "Berlin, Germany".to_string(),
        language: "English".to_string(),
        programming_language: "TypeScript".to_string(),
        description: "Build modern web applications and developer tools".to_string(),
        url: "https://vercel.com/careers".to_string(),
    }
}

async fn simulate_random_work() {
    sleep(Duration::from_millis(rand::random::<u64>() % 1000)).await;
}
