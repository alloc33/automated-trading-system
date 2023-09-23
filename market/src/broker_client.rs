use std::sync::Arc;

use anyhow::Result;
use apca::{api::v2::account, Client as AlpacaClient};
use thiserror::Error as ThisError;

use crate::{
    api::objects::Account,
    strategy_manager::{Broker, Order},
};

pub struct Clients {
    pub alpaca: Arc<AlpacaClient>,
}

impl Clients {
    pub fn new(alpaca: AlpacaClient) -> Self {
        Self {
            alpaca: Arc::new(alpaca),
        }
    }
}

#[derive(Debug, ThisError)]
pub enum BrokerError {
    #[error("Alpaca error: {0}")]
    AlpacaError(String),
}

#[axum::async_trait]
pub trait BrokerClient: Send + Sync {
    async fn get_account(&self) -> Result<Account, BrokerError>;
    async fn get_positions(&self) -> Result<(), BrokerError>;
    async fn get_orders(&self) -> Result<(), BrokerError>;
    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), BrokerError>;
    async fn cancel_order(&self) -> Result<(), BrokerError>;
    async fn cancel_all_orders(&self) -> Result<(), BrokerError>;
}

#[axum::async_trait]
impl BrokerClient for Arc<AlpacaClient> {
    async fn get_account(&self) -> Result<Account, BrokerError> {
        let result = self
            .issue::<account::Get>(&())
            .await
            .map_err(|e| BrokerError::AlpacaError(e.to_string()))?;

        Ok(Account::AlpacaAccount(result))
    }
    async fn get_positions(&self) -> Result<(), BrokerError> {
        Ok(())
    }
    async fn get_orders(&self) -> Result<(), BrokerError> {
        Ok(())
    }

    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), BrokerError> {
        Ok(())
    }
    async fn cancel_order(&self) -> Result<(), BrokerError> {
        Ok(())
    }
    async fn cancel_all_orders(&self) -> Result<(), BrokerError> {
        Ok(())
    }
}
