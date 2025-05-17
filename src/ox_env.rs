use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

use tokio::sync::oneshot;

use crate::repository::{JobRepo, JobRepoConfig};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OxConfig {
    host: IpAddr,
    port: u16,
    #[serde(flatten)]
    db_config: JobRepoConfig,
}

// todo extract out to own module
pub struct OxApp {
    pub socket_addr: SocketAddr,
    pub repo: JobRepo,
    pub shutdown: oneshot::Receiver<()>,
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
}
