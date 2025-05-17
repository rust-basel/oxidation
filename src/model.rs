use std::fmt::Display;

use axum::http::Uri;
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct JobId(i64);

impl Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(sqlx::Type, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct JobUri(String);

impl JobUri {
    pub fn new(uri: &Uri) -> Self {
        Self(uri.to_string())
    }
}

impl Display for JobUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<JobUri> for Uri {
    fn from(value: JobUri) -> Self {
        value.0.parse::<Uri>().unwrap_or_else(|err| {
            panic!(
                "JobUri should be guaranteed to wrap a Uri but had {}: {err}",
                value.0
            )
        })
    }
}

impl From<Uri> for JobUri {
    fn from(value: Uri) -> Self {
        JobUri(value.to_string())
    }
}

#[derive(sqlx::Decode, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Job {
    pub id: JobId,
    pub uri: JobUri,
    pub title: Option<String>,
    pub preface: Option<String>,
    pub description: Option<String>,
}

impl Job {
    pub fn title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }

    pub fn preface(&self) -> String {
        self.preface.clone().unwrap_or_default()
    }

    pub fn description(&self) -> String {
        self.description.clone().unwrap_or_default()
    }

    pub fn uri(&self) -> String {
        self.uri.to_string()
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Limit {
    pub page_size: Option<u8>,
    pub page: Option<u32>,
}

impl Limit {
    pub fn page_size_default(&self) -> u32 {
        self.page_size.unwrap_or(10) as u32
    }

    pub fn page_default(&self) -> u32 {
        self.page.unwrap_or(0)
    }
    pub fn offset(&self) -> u32 {
        self.page_default() * self.page_size_default()
    }
}
