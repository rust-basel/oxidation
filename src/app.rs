use anyhow::Context;
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

impl OxConfig {
    pub fn new(host: IpAddr, port: u16, db_config: JobRepoConfig) -> Self {
        Self {
            host,
            port,
            db_config,
        }
    }

    pub fn init() -> anyhow::Result<OxConfig> {
        let (host, port, db_url) = ox_environment::init();

        let ip: IpAddr = host.parse().context("Invalid IP address")?;
        let port: u16 = port.parse().context("Invalid port")?;
        let db_config = JobRepoConfig::new(db_url);

        Ok(OxConfig::new(ip, port, db_config))
    }
}

// todo extract out to own module
pub struct Oxidation {
    pub socket_addr: SocketAddr,
    pub repo: JobRepo,
    pub shutdown: oneshot::Receiver<()>,
}

impl Oxidation {
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

mod ox_environment {

    const DEFAULT_HOST: &str = "localhost";
    const DEFAULT_PORT: &str = "8120";
    const DEFAULT_DB_URL: &str = "sqlite:data/dev.db";

    const HOST_KEY: &str = "OX_HOST";
    const PORT_KEY: &str = "OX_PORT";
    const DB_URL_KEY: &str = "OX_DB_URL";
    const SQLX_URL_KEY: &str = "OX_SQLX_URL";

    use log::{debug, info, warn};
    use std::env;
    pub fn init() -> (String, String, String) {
        let host = env::var(HOST_KEY).unwrap_or(DEFAULT_HOST.to_string());
        let port: String = env::var(PORT_KEY).unwrap_or(DEFAULT_PORT.to_string());
        let db_url = env::var(DB_URL_KEY).unwrap_or(DEFAULT_DB_URL.to_string());

        let is_default_host = host == DEFAULT_HOST;
        let is_default_port = port == DEFAULT_PORT;
        let is_default_db_url = db_url == DEFAULT_DB_URL;

        if is_default_host {
            info!("using default host {}", host);
        } else {
            debug!("using host {}", host);
        }
        if is_default_port {
            info!("using default port {}", port);
        } else {
            debug!("using port {}", port);
        }
        if is_default_db_url {
            info!("using default database url {}", db_url);
        } else {
            debug!("using database url {}", db_url);
        }

        let sqlx_setup_missing = env::var(SQLX_URL_KEY).is_err();

        if sqlx_setup_missing && is_default_host {
            warn!("setting sqlx {} to {}", SQLX_URL_KEY, db_url);
            unsafe {
                env::set_var(SQLX_URL_KEY, db_url.clone());
            }
        }

        (host, port, db_url)
    }
}
