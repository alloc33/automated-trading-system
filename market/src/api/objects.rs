use apca::api::v2::{
    account::Account as AlpacaAccount,
    asset::Asset as AlpacaAsset,
    order::{ChangeReq as AlpacaOrderUpdateReq, Order as AlpacaOrder, OrderReq as AlpacaNewOrder},
    orders::OrdersReq as AlpacOrdersReq,
    position::Position as AlpacaPosition,
};
use serde::{Deserialize, Serialize};

use crate::{clients::BrokerClient, App};

pub trait GetBroker {
    fn broker(&self) -> Broker;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
}

#[derive(Debug, Serialize)]
pub enum Account {
    AlpacaAccount(AlpacaAccount),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetClass {
    #[serde(rename = "us_equity")]
    UsEquity,
    #[serde(rename = "crypto")]
    Crypto,
}

#[derive(Debug, Serialize)]
pub enum Asset {
    AlpacaAsset(AlpacaAsset),
}

#[derive(Debug, Serialize)]
pub enum Order {
    AlpacaOrder(AlpacaOrder),
}

#[derive(Debug, Deserialize)]
pub enum OrdersRequest {
    AlpacaOrders(AlpacOrdersReq),
}

#[derive(Debug, Deserialize)]
pub enum NewOrder {
    AlpacaNewOrder(AlpacaNewOrder),
}

#[derive(Debug, Deserialize)]
pub enum UpdateOrder {
    AlpacaUpdateOrder(AlpacaOrderUpdateReq),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Position {
    AlpacaPosition(AlpacaPosition),
}

impl From<AssetClass> for apca::api::v2::asset::Class {
    fn from(value: AssetClass) -> Self {
        match value {
            AssetClass::UsEquity => apca::api::v2::asset::Class::UsEquity,
            AssetClass::Crypto => apca::api::v2::asset::Class::Crypto,
        }
    }
}

impl Broker {
    pub fn get_client<'a>(&self, app: &'a App) -> &'a impl BrokerClient {
        match self {
            Broker::Alpaca => &app.clients.alpaca,
        }
    }
}

impl GetBroker for OrdersRequest {
    fn broker(&self) -> Broker {
        match self {
            OrdersRequest::AlpacaOrders(_) => Broker::Alpaca,
        }
    }
}
