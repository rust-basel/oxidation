use anyhow::Context;
use anyhow::{Result, bail};
use log::info;
use tokio::{signal, sync::oneshot};

use log::warn;

use axum::{Extension, Router};
use tokio::net::TcpListener;

mod app;
mod assets;
mod health;
mod http_types;
mod jobs;
mod model;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config: app::OxConfig = app::OxConfig::init()?;

    let (app, shutdown) = app::Oxidation::new(config)
        .await
        .context("failed to create app")?;

    let shutdown = tokio::spawn(handle_shutdown(shutdown));
    let serve = Box::pin(serve(app));

    match futures::future::select(shutdown, serve).await {
        futures::future::Either::Left((shutdown, serve)) => match shutdown {
            Ok(_) => serve.await?,
            Err(err) => {
                return Err(err).context(
                    "Tried to shutdown cleanly with signal, but received error. Shutting down",
                );
            }
        },
        futures::future::Either::Right((serve, _)) => serve?,
    }

    Ok(())
}

async fn handle_shutdown(shutdown: oneshot::Sender<()>) -> Result<()> {
    signal::ctrl_c()
        .await
        .context("failed to listen for event")?;
    info!("Received shutdown signal, sending shutdown message to server task");
    if shutdown.send(()).is_err() {
        bail!("Failed to send shutdown signal. Receiver must have terminated")
    }
    Ok(())
}

pub async fn serve(app: app::Oxidation) -> anyhow::Result<()> {
    let app::Oxidation {
        socket_addr,
        repo,
        shutdown,
    } = app;
    let routes = Router::new()
        .merge(health::router())
        .merge(jobs::router())
        .merge(assets::router())
        .layer(Extension(repo));

    let listener = TcpListener::bind(socket_addr)
        .await
        .context("failed to bind socket")?;
    info!("server bound to {socket_addr}");
    let serve = axum::serve(listener, routes);
    tokio::select!(
        serve = serve => match serve {
            Ok(()) => {
                info!("Serving task terminated but without error")
            }
                Err(err) => {
                warn!("Serving task terminated with error: {err}");
                bail!("Shutting down ")
            }
        },
        _ = shutdown => {
            info!("Received shutdown signal. Shutting down gracefully");

        }
    );
    Ok(())
}
