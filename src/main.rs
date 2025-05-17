use anyhow::Context;
use anyhow::{Result, bail};
use config::Config;
use log::info;
use ox_env::{OxApp, OxConfig};
use tokio::{signal, sync::oneshot};

mod assets;
mod handlers;
mod health;
mod http_types;
mod jobs;
mod model;
mod ox_env;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config: OxConfig = Config::builder()
        .add_source(config::File::with_name("config/default.toml").required(false))
        .add_source(config::File::with_name("config/config.toml").required(false))
        .add_source(config::Environment::with_prefix("OX"))
        .build()
        .context("failed to build config")?
        .try_deserialize()
        .context("failed to parse config")?;

    let (app, shutdown) = OxApp::new(config).await.context("failed to create app")?;

    let shutdown = tokio::spawn(handle_shutdown(shutdown));
    let serve = Box::pin(app.serve());

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
