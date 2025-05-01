use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6};

use axum::{
    Extension, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use tracing_log::log::{info, warn};

use crate::repository::{JobRepo, JobRepoConfig};
use anyhow::{Context, Result, bail};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RustJobsConfig {
    host: IpAddr,
    port: u16,
    #[serde(flatten)]
    db_config: JobRepoConfig,
}

pub struct RustJobsApp {
    socket_addr: SocketAddr,
    repo: JobRepo,
    shutdown: oneshot::Receiver<()>,
}

impl RustJobsApp {
    pub async fn new(
        RustJobsConfig {
            host,
            port,
            db_config,
        }: RustJobsConfig,
    ) -> Result<(Self, oneshot::Sender<()>)> {
        let (tx, shutdown) = oneshot::channel();
        let socket_addr = match host {
            IpAddr::V4(ipv4_addr) => SocketAddr::V4(SocketAddrV4::new(ipv4_addr, port)),
            IpAddr::V6(ipv6_addr) => SocketAddr::V6(SocketAddrV6::new(ipv6_addr, port, 0, 0)),
        };
        let repo = JobRepo::new(db_config).await?;
        Ok((
            Self {
                socket_addr,
                repo,
                shutdown,
            },
            tx,
        ))
    }

    pub async fn serve(self) -> Result<()> {
        let Self {
            socket_addr,
            repo,
            shutdown,
        } = self;
        let routes = Router::new()
            .route("/jobs", get(crate::handlers::get_jobs))
            .route("/api/jobs", post(crate::handlers::create_job))
            .route("/api/jobs/{id}", get(crate::handlers::get_job))
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
}
