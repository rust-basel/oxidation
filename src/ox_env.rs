use std::env;

pub struct Configuration {
    address: String,
    port: String,
}

impl Configuration {
    pub fn new(address: String, port: String) -> Self {
        Self { address, port }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn port(&self) -> &str {
        &self.port
    }
}

pub fn init() -> Configuration {
    let address = env::var("OXIDATION_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("OXIDATION_PORT").unwrap_or("8000".to_string());

    Configuration::new(address, port)
}

pub fn host(config: Configuration) -> String {
    format!("{}:{}", config.address(), config.port())
}
