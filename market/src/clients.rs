use std::sync::Arc;

use anyhow::Result;
use apca::{api::v2::account, Client as AlpacaClient};
use thiserror::Error as ThisError;

use crate::{
    api::objects::Account,
    strategy_manager::{Exchange, Order},
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
pub enum ExchangeClientError {
    #[error("Alpaca error: {0}")]
    AlpacaError(String),
}

#[axum::async_trait]
pub trait ExchangeClient: Send + Sync {
    async fn get_account(&self) -> Result<Account, ExchangeClientError>;
    async fn get_positions(&self) -> Result<(), ExchangeClientError>;
    async fn get_orders(&self) -> Result<(), ExchangeClientError>;
    async fn place_order(&self, order: &Order, exchange: &Exchange) -> Result<(), ExchangeClientError>;
    async fn cancel_order(&self) -> Result<(), ExchangeClientError>;
    async fn cancel_all_orders(&self) -> Result<(), ExchangeClientError>;
}

#[axum::async_trait]
impl ExchangeClient for Arc<AlpacaClient> {
    async fn get_account(&self) -> Result<Account, ExchangeClientError> {
        let result = self
            .issue::<account::Get>(&())
            .await
            .map_err(|e| ExchangeClientError::AlpacaError(e.to_string()))?;

        Ok(Account::AlpacaAccount(result))
    }
    async fn get_positions(&self) -> Result<(), ExchangeClientError> {
        Ok(())
    }
    async fn get_orders(&self) -> Result<(), ExchangeClientError> {
        Ok(())
    }

    async fn place_order(&self, order: &Order, exchange: &Exchange) -> Result<(), ExchangeClientError> {
        Ok(())
    }
    async fn cancel_order(&self) -> Result<(), ExchangeClientError> {
        Ok(())
    }
    async fn cancel_all_orders(&self) -> Result<(), ExchangeClientError> {
        Ok(())
    }
}
