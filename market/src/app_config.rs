use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;

use crate::strategy::Strategy;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Brokers {
    pub alpaca: Alpaca,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Alpaca {
    pub apca_api_key_id: String,
    pub apca_api_secret_key: String,
    pub apca_api_base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub database: Database,
    pub brokers: Brokers,
    pub strategies: Vec<Strategy>,
}

impl AppConfig {
    pub fn build() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("market/config/default"))
            .add_source(File::with_name(&format!("market/config/{}", run_mode)).required(false))
            .build()?;

        println!("debug: {:?}", config.get_bool("debug"));
        println!("database: {:?}", config.get::<String>("database.url"));

        config.try_deserialize()
    }

    pub fn build_for_test() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("../market/config/default"))
            .add_source(File::with_name(&format!("../market/config/{}", run_mode)).required(false))
            .build()?;

        println!("debug: {:?}", config.get_bool("debug"));
        println!("database: {:?}", config.get::<String>("database.url"));

        config.try_deserialize()
    }
}
