use std::sync::Arc;

use apca::{ApiInfo, Client as AlpacaClient};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use thiserror::Error as ThisError;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use uuid::Uuid;
use uuid7::uuid7;

use crate::{
    api::alert::{WebhookAlertData, TradeSignal, AlertType},
    trade_executor::TradeExecutor, App, broker::Broker,
};

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

// #[derive(Debug, ThisError)]
// pub enum TradeError {
//     #[error("{0}")]
//     InsufficientFunds(String),
//     #[error("Order max retries reached. {0}")]
//     MaxRetriesReached(Order),
// }

pub struct StrategyManager {
    app_state: Arc<App>,
    alpaca_client: AlpacaClient,
    trade_executor: TradeExecutor,
}

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub broker: Broker,
    pub ticker: String,
    pub order_type: AlertType,
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

    pub async fn process_trade_signal(
        &self,
        trade_signal: TradeSignal,
    ) -> Result<(), StrategyManagerError> {
        let order = Order {
            id: uuid7::new_v7(),
            broker: trade_signal.strategy.broker,
            ticker: trade_signal.ticker,
            order_type: trade_signal.alert_type
        };

        // TODO: Retry
        // let result = self.trade_executor.execute_order(&order);

        Ok(())
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {} }}", self.id)
    }
}
