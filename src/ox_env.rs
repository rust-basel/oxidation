use std::net::IpAddr;

use anyhow::{Context, bail};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

use axum::{
    Extension, Router,
    routing::{delete, post, put},
};
use tokio::{net::TcpListener, sync::oneshot};

use crate::{
    assets, health, jobs,
    repository::{JobRepo, JobRepoConfig},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OxConfig {
    host: IpAddr,
    port: u16,
    #[serde(flatten)]
    db_config: JobRepoConfig,
}

pub struct OxApp {
    socket_addr: SocketAddr,
    repo: JobRepo,
    shutdown: oneshot::Receiver<()>,
}

impl OxApp {
    pub async fn new(
        OxConfig {
            host,
            port,
            db_config,
        }: OxConfig,
    ) -> anyhow::Result<(Self, oneshot::Sender<()>)> {
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

    pub async fn serve(self) -> anyhow::Result<()> {
        let Self {
            socket_addr,
            repo,
            shutdown,
        } = self;
        let routes = Router::new()
            .route("/api/jobs", put(crate::handlers::create_job))
            .route("/api/jobs/{job_id}", post(crate::handlers::update_job))
            .route("/api/jobs/{job_id}", delete(crate::handlers::delete_job))
            .layer(Extension(repo));

        let wiht_view = routes
            .merge(health::router())
            .merge(jobs::router())
            .merge(assets::router());

        let listener = TcpListener::bind(socket_addr)
            .await
            .context("failed to bind socket")?;
        info!("server bound to {socket_addr}");
        let serve = axum::serve(listener, wiht_view);
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
