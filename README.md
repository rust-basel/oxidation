# Oxidation

This project is an job posting application for Jobs with Rust.

## API

Checkout api_tests and jump into any hurl file [here](https://github.com/rust-basel/oxidation/tree/main/api_tests)

## Development
A simple development database is used for testing migrations and using sqlxs compile time query 
verification in `data/dev.db`. This is configured in the committed `.env` file. If you do not have an `.env` yet, you can
copy the `.env.example` as starter.
It's schema should always be up-to-date.

### SQLX

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

### Cargo

A default configuration is configured in `config/default.toml`. 
```sh
cargo run
```

This can be overridden with `config/config.toml` and environment variables preceded by `RUST_JOBS_`, e.g.

```sh
OX_PORT=9876 cargo run
```

will override the port configured in the default config.

### Docker Compose

```sh
docker-compose up
```
