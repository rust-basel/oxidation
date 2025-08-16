use crate::scraper::JobPosting;

pub fn render_result(job_postings: Vec<JobPosting>) {
    for posting in job_postings {
        println!("Title: {}", posting.title);
        println!("Location: {}", posting.location);
        println!("Language: {}", posting.language);
        println!("Company: {}", posting.company);
        println!("Programming Language: {}", posting.programming_language);
        println!("Description: {}", posting.description);
        println!("URL: {}", posting.url);
        println!();
    }
}
