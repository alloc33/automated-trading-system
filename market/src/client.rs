use std::sync::Arc;

use apca::Client as AlpacaClient;

use crate::strategy_manager::{Broker, Order};

#[axum::async_trait]
pub trait BrokerClient: Send + Sync {
    async fn get_account(&self) -> Result<(), ()>;
    async fn get_positions(&self) -> Result<(), ()>;
    async fn get_orders(&self) -> Result<(), ()>;
    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), ()>;
    async fn cancel_order(&self) -> Result<(), ()>;
    async fn cancel_all_orders(&self) -> Result<(), ()>;
}

pub struct Clients {
    pub alpaca: Arc<AlpacaClient>,
}

impl Clients {
    pub fn new(alpaca: AlpacaClient) -> Self {
        Self {
            alpaca: Arc::new(alpaca)
        }
    }
}

#[axum::async_trait]
impl BrokerClient for Arc<AlpacaClient> {
    async fn get_account(&self) -> Result<(), ()> {
        Ok(())
    }
    async fn get_positions(&self) -> Result<(), ()> {
        Ok(())
    }
    async fn get_orders(&self) -> Result<(), ()> {
        Ok(())
    }

    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), ()> {
        Ok(())
    }
    async fn cancel_order(&self) -> Result<(), ()> {
        Ok(())
    }
    async fn cancel_all_orders(&self) -> Result<(), ()> {
        Ok(())
    }
}

