use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub address: Address,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("settings.json"))
            .set_default("address.host", "127.0.0.1")?
            .set_default("address.port", 3000)?
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
