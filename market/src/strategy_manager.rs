use std::sync::Arc;

use apca::{ApiInfo, Client as AlpacaClient};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use thiserror::Error as ThisError;
use tokio::{
    sync::mpsc::UnboundedReceiver,
    time::{sleep, Duration},
};
use tracing::{error, info};
use uuid::Uuid;
use uuid7::uuid7;

use crate::{
    api::alert::{AlertType, TradeSignal, WebhookAlertData},
    clients::{ExchangeClient, Clients},
    App,
};

pub async fn process_trade_signal(
    client: impl ExchangeClient,
    signal: TradeSignal,
) -> Result<(), ()> {
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Exchange {
    Alpaca,
}

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub ticker: String,
    pub order_type: AlertType,
}

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

#[derive(Debug, ThisError)]
pub enum TradeError {
    #[error("{0}")]
    InsufficientFunds(String),
    #[error("Order max retries reached. {0}")]
    MaxRetriesReached(Order),
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {} }}", self.id)
    }
}
