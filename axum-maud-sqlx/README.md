# Axum - Maud - Sqlx PoC
This is a simple PoC with the basics of crud for some jobs. Right now it just supports ids and URIs so 
we can link to a source. In the future we could support more metadata.

This server implements:
#### Get all jobs 
with optional `page` (default 0) and `page_size` (default 10) query parameters. Displays all jobs with
links to the job page (below) and the source URI.
```
GET  jobs?page={PAGE NUMBER}&page_size={PAGE_SIZE}
```
#### Get a single job 
A simple page with a back to the full jobs page and the same details as above for a given job
```
GET  jobs/{job_id}
```

#### Create a new post
Create a new job with a json string of for the URI
```
PUT api/jobs 
  -H 'Content-Type: application/json'  
  --data '"http://rust-basel.ch/some-job-3.html&query_param=value&whats_this"'
```

#### Update a post
```
POST api/jobs/{job_id}
  -H 'Content-Type: application/json'  
  --data '"http://rust-basel.ch/some-job-3.html&query_param=value&whats_this"'
```

#### Delete a post 
Delet a job post
```
DELETE api/jobs/{job_id}
```

## Development
A simple development database is used for testing migrations and using sqlxs compile time query 
verification in `data/dev.db`. This is configured in the committed `.env` file. It's schema 
should always be up-to-date.

To add a migration run (making sure you have the [sqlx-cli](https://lib.rs/crates/sqlx-cli))
```sh
sqlx migrate new {MIGRATION_NAME}
```
This will create a new SQL file in `migrations/` prefixed with a timestamp. Add your migration there
and run 
```sh
sqlx migrate run
```
to migrate the dev db. Now you should be able to copile-time check your queries with `sqlx::query!` 
and `sqlx::query_as!`.

## Running
A default configuration is configured in `config/default.toml`. This can be overridden with 
`config/config.toml` and environment variables preceded by `RUST_JOBS_`, e.g.
```sh
RUST_JOBS_PORT=9876 cargo run
```
will override the port configured in the default config.
