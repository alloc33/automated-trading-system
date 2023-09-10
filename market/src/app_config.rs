use config::{Config, ConfigError, File};
use serde::Deserialize;
use uuid::Uuid;

use crate::strategy_manager::broker::Broker;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub broker: Broker,
    pub max_event_retries: u8,
    pub event_retry_delay: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub database: Database,
    pub strategies: Vec<Strategy>,
}

impl AppConfig {
    pub fn build() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("market/config.toml"))
            .build()?;

        config.try_deserialize()
    }
}

pub trait StrategySelector {
    fn select(&self, id: Uuid) -> Strategy;
}
