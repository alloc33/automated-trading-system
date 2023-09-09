use config::{Config, ConfigError, File};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Common {
    pub api_key: String,
    pub trade_signal_max_retries: u8,
    pub trade_signal_retry_delay: Decimal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub broker: Broker,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: Database,
    pub common: Common,
    pub strategies: Vec<Strategy>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
    // TODO: ?add more brokers
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
