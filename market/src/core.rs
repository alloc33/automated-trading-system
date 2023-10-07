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
    api::{
        alert::{SignalType, WebhookAlertData},
        objects::Broker,
    },
    clients::{BrokerClient, Clients},
    trade_signal::TradeSignal,
    App,
};

pub struct Core;

impl Core {
    // NOTE: ?scheduled tasks
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub async fn process_trade_signal<C: BrokerClient>(
        &self,
        client: C,
        trade_signal: TradeSignal,
    ) -> Result<(), ()> {
        // let order_request = client.create_order_request
        // let order = client.create_order(new_order_req);
        // match trade_signal.signal_type {
        //     SignalType::OpenLong => {}
        //     SignalType::OpenShort => {}
        //     SignalType::StopLossUpdate => {}
        // };
        Ok(())
    }

    async fn order_request(&self, broker: Broker) -> Result<(), ()> {
        Ok(())
    }
}

// #[derive(Debug)]
// pub struct Order {
//     pub id: Uuid,
//     pub ticker: String,
//     pub order_type: AlertType,
// }

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
    // #[error("Order max retries reached. {0}")]
    // MaxRetriesReached(Order),
}

// impl std::fmt::Display for Order {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{ id: {} }}", self.id)
//     }
// }
