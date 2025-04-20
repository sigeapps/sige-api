use std::net::SocketAddr;

use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Logging {
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub logging: Logging,
    pub address: Address,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config/settings.json"))
            .set_default("address.host", "127.0.0.1")?
            .set_default("address.port", 3000)?
            .set_default("log_level", "debug")?
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
