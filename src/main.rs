use std::env;

use axum::{Router, routing::get};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let address = env::var("OXIDATION_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("OXIDATION_PORT").unwrap_or("8000".to_string());

    let host = format!("{}:{}", address, port);
    //
    let app = Router::new().route("/healthz", get(|| async { "" }));

    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    info!(target: "server", "started {host}");
    axum::serve(listener, app).await.unwrap();
}
