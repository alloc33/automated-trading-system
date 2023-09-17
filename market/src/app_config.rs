use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use uuid::Uuid;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Alpaca {
    pub apca_api_key_id: String,
    pub apca_api_secret_key: String,
    pub apca_api_base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub broker: Broker,
    pub max_order_retries: u8,
    pub order_retry_delay: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub database: Database,
    pub alpaca: Alpaca,
    pub strategies: Vec<Strategy>,
}

impl AppConfig {
    pub fn build() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("market/config/default"))
            .add_source(
                File::with_name(&format!("market/config/{}", run_mode))
                    .required(false),
            )
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", config.get_bool("debug"));
        println!("database: {:?}", config.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        config.try_deserialize()
    }
}
