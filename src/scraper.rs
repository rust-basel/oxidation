use crate::config::Config;

pub struct JobPosting {
    pub title: String,
    pub company: String,
    pub location: String,
    pub language: String,
    pub programming_language: String,
    pub description: String,
    pub url: String,
}

pub async fn scrape_jobs_postings(_config: &Config) -> Vec<JobPosting> {
    // fake jobs
    vec![
        JobPosting {
            title: "Software Engineer".to_string(),
            company: "Google".to_string(),
            location: "Mountain View, CA".to_string(),
            language: "English".to_string(),
            programming_language: "Python".to_string(),
            description: "Design and develop software solutions".to_string(),
            url: "https://www.google.com/jobs".to_string(),
        },
        JobPosting {
            title: "Data Scientist".to_string(),
            company: "Amazon".to_string(),
            location: "Seattle, WA".to_string(),
            language: "English".to_string(),
            programming_language: "R".to_string(),
            description: "Analyze and interpret data".to_string(),
            url: "https://www.amazon.jobs".to_string(),
        },
    ]
}
