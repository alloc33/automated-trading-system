use std::sync::Arc;

use anyhow::Result;
use apca::{
    api::v2::{account as apca_account, asset as apca_asset, assets as apca_assets},
    Client as AlpacaClient,
};
use thiserror::Error as ThisError;

use crate::{
    api::objects::{Account, Asset, AssetClass},
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
pub enum BrokerClientError {
    #[error("Alpaca error: {0}")]
    AlpacaError(String),
}

#[axum::async_trait]
pub trait BrokerClient: Send + Sync {
    async fn get_account(&self) -> Result<Account, BrokerClientError>;
    async fn get_asset(&self, symbol: String) -> Result<Asset, BrokerClientError>;
    async fn get_assets(&self, class: AssetClass) -> Result<Vec<Asset>, BrokerClientError>;
    async fn get_positions(&self) -> Result<(), BrokerClientError>;
    async fn get_orders(&self) -> Result<(), BrokerClientError>;
    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), BrokerClientError>;
    async fn cancel_order(&self) -> Result<(), BrokerClientError>;
    async fn cancel_all_orders(&self) -> Result<(), BrokerClientError>;
}

#[axum::async_trait]
impl BrokerClient for Arc<AlpacaClient> {
    async fn get_account(&self) -> Result<Account, BrokerClientError> {
        let result = self.issue::<apca_account::Get>(&()).await.map_err(|e| {
            dbg!(&e);
            BrokerClientError::AlpacaError(e.to_string())
        })?;

        Ok(Account::AlpacaAccount(result))
    }
    async fn get_asset(&self, symbol: String) -> Result<Asset, BrokerClientError> {
        let result = self
            .issue::<apca_asset::Get>(&apca_asset::Symbol::Sym(symbol))
            .await
            .map_err(|e| BrokerClientError::AlpacaError(e.to_string()))?;
        Ok(Asset::AlpacaAsset(result))
    }

    async fn get_assets(&self, class: AssetClass) -> Result<Vec<Asset>, BrokerClientError> {
        let class = match class {
            AssetClass::UsEquity => apca_asset::Class::UsEquity,
            AssetClass::Crypto => apca_asset::Class::Crypto,
        };

        let asset_req = apca_assets::AssetsReq {
            status: apca_asset::Status::Active,
            class,
        };

        let result = self
            .issue::<apca_assets::Get>(&asset_req)
            .await
            .map_err(|e| BrokerClientError::AlpacaError(e.to_string()))?;

        Ok(result.into_iter().map(Asset::AlpacaAsset).collect())
    }

    async fn get_positions(&self) -> Result<(), BrokerClientError> {
        Ok(())
    }
    async fn get_orders(&self) -> Result<(), BrokerClientError> {
        Ok(())
    }

    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), BrokerClientError> {
        Ok(())
    }
    async fn cancel_order(&self) -> Result<(), BrokerClientError> {
        Ok(())
    }
    async fn cancel_all_orders(&self) -> Result<(), BrokerClientError> {
        Ok(())
    }
}
