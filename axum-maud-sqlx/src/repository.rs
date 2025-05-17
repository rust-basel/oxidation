use anyhow::Context;
use anyhow::Result;
use axum::http::Uri;
use serde::{Deserialize, Serialize};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tracing_log::log::info;

use crate::model::Job;
use crate::model::Limit;
use crate::model::{JobId, JobUri};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JobRepoConfig {
    db_path: String,
}

#[derive(Clone, Debug)]
pub struct JobRepo {
    pool: SqlitePool,
}

impl JobRepo {
    pub async fn new(JobRepoConfig { db_path }: JobRepoConfig) -> Result<Self> {
        // todo db_connection logging
        let options = SqliteConnectOptions::new().filename(&db_path);
        info!("Connecting to db at {db_path}");
        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .context(
                format!("Failed to connect to database file: {db_path}. Is it really there? is the path correct?")
            )?;
        info!("running migrations");
        sqlx::migrate!()
            .run(&pool)
            .await
            .context("failed to run migrations")?;
        info!("migrations completed successfully");
        Ok(Self { pool })
    }

    pub async fn create(&self, uri: &Uri) -> Result<Job> {
        let uri_str = uri.to_string();
        sqlx::query_as!(
            Job,
            r#"
                INSERT INTO 
                    job (uri) VALUES (?1)
                ON CONFLICT (uri) DO UPDATE SET uri = (?1)
                RETURNING id as "id: JobId", uri as "uri: JobUri";
            "#,
            uri_str
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert")
    }

    pub async fn get_page(&self, limit: Limit) -> Result<Vec<Job>> {
        let offset = limit.offset();
        let page_size = limit.page_size_default();
        sqlx::query_as!(
            Job,
            r#"
                SELECT 
                    id as "id: JobId",
                    uri as "uri: JobUri" 
                FROM job LIMIT ?1 OFFSET ?2;
            "#,
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to execute query for page of jobs")
    }

    pub async fn delete(&self, id: JobId) -> Result<Option<Job>> {
        sqlx::query_as!(
            Job,
            r#"
                DELETE FROM job 
                WHERE id = ?1
                RETURNING
                    id as "id: JobId",
                    uri as "uri: JobUri";
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!("Failed to execute query for job with id {id}"))
    }

    pub async fn update(&self, id: JobId, uri: &JobUri) -> Result<Option<Job>> {
        sqlx::query_as!(
            Job,
            r#"
                UPDATE job SET uri = ?1
                WHERE id = ?2
                RETURNING
                    id as "id: JobId",
                    uri as "uri: JobUri";
            "#,
            uri,
            id
        )
        .fetch_optional(&self.pool)
        .await
        // todo: Unique constraint violation handling, should bubble up to bad request
        .context(format!("Failed to execute query for job with id {id}"))
    }

    pub async fn get_one(&self, id: JobId) -> Result<Option<Job>> {
        sqlx::query_as!(
            Job,
            r#"
                SELECT 
                    id as "id: JobId",
                    uri as "uri: JobUri" 
                FROM job WHERE id = ?1;
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!("Failed to execute query for job with id {id}"))
    }
}

#[cfg(test)]
mod test {
    use axum::http::Uri;
    use sqlx::{Connection, SqliteConnection, SqlitePool};

    use crate::{
        model::{JobUri, Limit},
        repository::JobRepo,
    };

    #[tokio::test]
    async fn test_conn() {
        let url = "sqlite:data/database.db";
        SqliteConnection::connect(url)
            .await
            .expect("Failed open file");
    }

    #[sqlx::test]
    async fn test_repo(pool: SqlitePool) {
        let repo = JobRepo { pool };
        let uri = "https://rust-basel.ch/rustacean"
            .parse::<Uri>()
            .expect("failed to parse Uri from test string");
        let job_uri: JobUri = uri.clone().into();
        let job = repo
            .create(&uri)
            .await
            .expect("failed to insert job in test");
        let job = repo
            .get_one(job.id)
            .await
            .expect("failed to execute retrieve job query")
            .expect("didn't find the job we just inserted for test");

        let jobs = repo
            .get_page(Limit {
                page_size: Some(10),
                page: None,
            })
            .await
            .expect("failed to get page for test");

        let empty = repo
            .get_page(Limit {
                page_size: Some(10),
                page: Some(1),
            })
            .await
            .expect("failed to get page for test");

        assert!(empty.is_empty());
        assert_eq!(jobs, vec![job.clone()]);
        assert_eq!(job_uri, job.uri)
    }
}
