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
    client::{BrokerClient, Clients},
    App, Event, events::{ActionType, Action},
};

pub async fn process_trade_signal(client: impl BrokerClient, signal: TradeSignal) -> Result<(), ()> {
    Ok(())
}

pub async fn process_market_action(
    client: impl BrokerClient,
    action: ActionType,
) -> Result<(), ()> {
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
}

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub ticker: String,
    pub order_type: AlertType,
}

// #[derive(Debug, ThisError)]
// pub enum StrategyManagerError {
//     #[error(transparent)]
//     ConfigError(#[from] ConfigError),
//     #[error(transparent)]
//     AlpacaClientError(#[from] apca::Error),
//     #[error("Unknown strategy - {0}")]
//     UnknownStrategy(String),
//     #[error("Unknown exchange - {0}")]
//     UnknownExchange(String),
//     #[error("Strategy {0} with id {1} is disabled")]
//     StrategyDisabled(String, String),
// }

// // #[derive(Debug, ThisError)]
// // pub enum TradeError {
// //     #[error("{0}")]
// //     InsufficientFunds(String),
// //     #[error("Order max retries reached. {0}")]
// //     MaxRetriesReached(Order),
// // }

// pub struct StrategyManager {
//     app_state: Arc<App>,
//     // clients: Vec<BrokerStruct>, // trade_executor: TradeExecutor,
// }

// impl StrategyManager {
//     pub fn new(
//         app_state: Arc<App>,
//         // trade_executor: TradeExecutor,
//     ) -> Result<Self, StrategyManagerError> {
//         // let alpaca_client = AlpacaClient::new(ApiInfo::from_parts(
//         //     &app_state.config.alpaca.apca_api_base_url,
//         //     &app_state.config.alpaca.apca_api_key_id,
//         //     &app_state.config.alpaca.apca_api_secret_key,
//         // )?);

//         // let clients = vec![BrokerStruct(Box::new(alpaca_client))];

//         Ok(Self {
//             app_state,
//             // clients, // trade_executor,
//         })
//     }

//     pub async fn process_trade_signal(
//         &self,
//         trade_signal: TradeSignal,
//     ) -> Result<(), StrategyManagerError> {
//         let order = Order {
//             id: uuid7::new_v7(),
//             ticker: trade_signal.ticker,
//             order_type: trade_signal.alert_type,
//         };

//         // match trade_signal.strategy.broker {
//         //     Broker::Alpaca => {
//         //         self.process_alpaca_order(order, trade_signal).await?;
//         //     }
//         // }

//         // // TODO: Retry
//         // let result = self
//         //     .trade_executor
//         //     .execute_order(&order, &trade_signal.strategy.broker)
//         //     .await;

//         Ok(())
//     }
// }

// impl std::fmt::Display for Order {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{ id: {} }}", self.id)
//     }
// }
