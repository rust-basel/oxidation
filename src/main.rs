use axum::Router;
use log::info;

mod health;
mod ox_env;

fn app() -> Router {
    Router::new().nest("/", health::router())
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
