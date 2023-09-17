use apca::Client as AlpacaClient;

use crate::strategy_manager::{Broker, Order};

pub struct TradeExecutor {
    pub alpaca_client: AlpacaClient,
}

impl TradeExecutor {
    pub fn new(alpaca_client: AlpacaClient) -> Self {
        Self { alpaca_client }
    }

    pub async fn execute_order(&self, order: &Order, broker: &Broker) -> Result<(), ()> {
        Ok(())
    }
}
