use axum::Router;
use log::info;

mod assets;
mod health;
mod http_types;
mod jobs;
mod ox_env;

fn app() -> Router {
    Router::new()
        .merge(health::router())
        .merge(jobs::router())
        .merge(assets::router())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = ox_env::init();

    let host = ox_env::host(config);

    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    info!(target: "server", "started {host}");
    axum::serve(listener, app()).await.unwrap();
}
