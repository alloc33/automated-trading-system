use std::sync::Arc;

use anyhow::Result;
use apca::{
    api::v2::{
        account as apca_account, asset as apca_asset, assets as apca_assets, order as apca_order,
        orders as apca_orders,
    },
    Client as AlpacaClient,
};
use thiserror::Error as ThisError;
use uuid::Uuid;

use crate::{
    api::objects::{Account, Asset, AssetClass, Order},
    objects::Broker,
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
    #[error("Alpaca request error: {0}")]
    AlpacaError(String),
}

#[axum::async_trait]
pub trait BrokerClient: Send + Sync {
    type OrdersRequest;

    async fn get_account(&self) -> Result<Account, BrokerClientError>;
    async fn get_asset(&self, symbol: String) -> Result<Asset, BrokerClientError>;
    async fn get_assets(&self, class: AssetClass) -> Result<Vec<Asset>, BrokerClientError>;
    async fn get_position(&self) -> Result<(), BrokerClientError>;
    async fn get_positions(&self) -> Result<(), BrokerClientError>;
    async fn get_order(&self, order_id: Uuid) -> Result<Order, BrokerClientError>;
    async fn get_orders(
        &self,
        brokder_orders: Self::OrdersRequest,
    ) -> Result<Vec<Order>, BrokerClientError>;
    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), BrokerClientError>;
    async fn cancel_order(&self) -> Result<(), BrokerClientError>;
    async fn cancel_all_orders(&self) -> Result<(), BrokerClientError>;
}

#[axum::async_trait]
impl BrokerClient for Arc<AlpacaClient> {
    type OrdersRequest = apca_orders::OrdersReq;

    async fn get_account(&self) -> Result<Account, BrokerClientError> {
        let result = self.issue::<apca_account::Get>(&()).await;

        if let Ok(account) = result {
            return Ok(Account::AlpacaAccount(account));
        } else {
            return Err(BrokerClientError::AlpacaError(format!("{result:?}")));
        }
    }

    async fn get_asset(&self, symbol: String) -> Result<Asset, BrokerClientError> {
        let result = self
            .issue::<apca_asset::Get>(&apca_asset::Symbol::Sym(symbol))
            .await;

        if let Ok(asset) = result {
            return Ok(Asset::AlpacaAsset(asset));
        } else {
            return Err(BrokerClientError::AlpacaError(format!("{result:?}")));
        }
    }

    async fn get_assets(&self, class: AssetClass) -> Result<Vec<Asset>, BrokerClientError> {
        let asset_req = apca_assets::AssetsReq {
            status: apca_asset::Status::Active,
            class: class.into(),
        };

        let result = self.issue::<apca_assets::Get>(&asset_req).await;

        if let Ok(assets) = result {
            return Ok(assets.into_iter().map(Asset::AlpacaAsset).collect());
        } else {
            return Err(BrokerClientError::AlpacaError(format!("{result:?}")));
        }
    }

    async fn get_position(&self) -> Result<(), BrokerClientError> {
        Ok(())
    }

    async fn get_positions(&self) -> Result<(), BrokerClientError> {
        Ok(())
    }

    async fn get_order(&self, order_id: Uuid) -> Result<Order, BrokerClientError> {
        let result = self
            .issue::<apca_order::Get>(&apca_order::Id(order_id))
            .await;

        if let Ok(order) = result {
            return Ok(Order::AlpacaOrder(order));
        } else {
            return Err(BrokerClientError::AlpacaError(format!("{result:?}")));
        }
    }

    async fn get_orders(
        &self,
        orders_request: Self::OrdersRequest,
    ) -> Result<Vec<Order>, BrokerClientError> {
        let result = self.issue::<apca_orders::Get>(&orders_request).await;

        if let Ok(orders) = result {
            return Ok(orders.into_iter().map(Order::AlpacaOrder).collect());
        } else {
            return Err(BrokerClientError::AlpacaError(format!("{result:?}")));
        }
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
