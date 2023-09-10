use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Alpaca {
    pub apca_api_key_id: String,
    pub apca_api_secret_key: String,
    pub apca_api_base_url: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub database: Database,
    pub alpaca: Alpaca
}

impl AppConfig {
    pub fn build() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("market/config.toml"))
            .build()?;

        config.try_deserialize()
    }
}
