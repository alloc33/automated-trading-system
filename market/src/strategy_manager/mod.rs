pub mod broker;
pub mod trade_error;

use std::sync::Arc;

use apca::{ApiInfo, Client as AlpacaClient};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use thiserror::Error as ThisError;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use trade_error::TradeError;
use uuid::Uuid;
use uuid7::uuid7;

use self::broker::Broker;
use crate::{api::alert::AlertData, events::Event, trade_executor::TradeExecutor, App};

#[derive(Debug, ThisError)]
pub enum StrategyManagerError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    AlpacaClientError(#[from] apca::Error),
    #[error("Unknown strategy - {0}")]
    UnknownStrategy(String),
    #[error("Unknown exchange - {0}")]
    UnknownExchange(String),
    #[error("Strategy {0} with id {1} is disabled")]
    StrategyDisabled(String, String),
}

pub struct StrategyManager {
    app_state: Arc<App>,
    alpaca_client: AlpacaClient,
    trade_executor: TradeExecutor,
}

#[derive(Debug)]
pub struct Order {
    id: Uuid,
    ticker: String,
}

impl StrategyManager {
    pub fn new(
        app_state: Arc<App>,
        trade_executor: TradeExecutor,
    ) -> Result<Self, StrategyManagerError> {
        let alpaca_client = AlpacaClient::new(ApiInfo::from_parts(
            &app_state.config.alpaca.apca_api_base_url,
            &app_state.config.alpaca.apca_api_key_id,
            &app_state.config.alpaca.apca_api_secret_key,
        )?);

        Ok(Self {
            app_state,
            alpaca_client,
            trade_executor,
        })
    }

    pub async fn process_trade_signal(&self, alert_data: AlertData) -> Result<(), StrategyManagerError> {
        Ok(())
    }

    fn create_order(&self, alert_data: &AlertData) -> Result<Order, StrategyManagerError> {
        // Validate strategy - check if strategy exists and it's enabled.
        // let validated_strategy = self
        //     .strategies
        //     .iter()
        //     .find(|strategy| strategy.id == alert_data.strategy_id)
        //     .ok_or_else(|| {
        //         StrategyManagerError::UnknownStrategy(alert_data.strategy_id.to_string())
        //     })?;

        // if !validated_strategy.enabled {
        //     return Err(StrategyManagerError::StrategyDisabled(
        //         validated_strategy.name.clone(),
        //         validated_strategy.id.to_string(),
        //     ));
        // }

        // TODO: Complete Order creation
        let order = Order {
            id: uuid7::new_v7(),
            ticker: alert_data.ticker.clone(),
        };

        Ok(order)
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {} }}", self.id)
    }
}
