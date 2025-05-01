use anyhow::{Context, Result, bail};
use config::Config;
use routing::{RustJobsApp, RustJobsConfig};
use tokio::{signal, sync::oneshot};
use tracing_log::log::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod model;
mod repository;
mod routing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .context("Failed to initializing tracing subscriber for logging")?;
    let config: RustJobsConfig = Config::builder()
        .add_source(config::File::with_name("config/default.toml"))
        .add_source(config::File::with_name("config/config.toml").required(false))
        .add_source(config::Environment::with_prefix("RUST_JOBS"))
        .build()
        .context("failed to build config")?
        .try_deserialize()
        .context("failed to parse config")?;
    let (app, shutdown) = RustJobsApp::new(config)
        .await
        .context("failed to create app")?;

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
