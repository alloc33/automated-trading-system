use apca::Client as AlpacaClient;

use crate::strategy_manager::{Broker, Order};

#[axum::async_trait]
trait BrokerClient {
    async fn get_account(&self) -> Result<(), ()>;
    async fn get_positions(&self) -> Result<(), ()>;
    async fn get_orders(&self) -> Result<(), ()>;
    // async fn get_order(&self) -> Result<(), ()>;
    async fn place_order(&self, order: &Order, broker: &Broker) -> Result<(), ()>;
    async fn cancel_order(&self) -> Result<(), ()>;
    async fn cancel_all_orders(&self) -> Result<(), ()>;
}

pub struct TradeExecutor;



// impl<C: MarketClient> TradeExecutor<T> {
//     pub fn new(alpaca_client: AlpacaClient) -> Self {
//         Self { alpaca_client }
//     }

//     pub async fn execute_order(&self, order: &Order, broker: &Broker) -> Result<(), ()> {
//         Ok(())
//     }
// }

#[axum::async_trait]
impl BrokerClient for AlpacaClient {
    async fn get_account(&self) -> Result<(), ()> {
        Ok(())
    }
    async fn get_positions(&self) -> Result<(), ()> {
        Ok(())
    }
    async fn get_orders(&self) -> Result<(), ()> {
        Ok(())
    }
    // async fn get_order(&self) -> Result<(), ()> {
    //     Ok(())
    // }
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

pub struct BrokerStruct(pub Box<dyn BrokerClient>);
