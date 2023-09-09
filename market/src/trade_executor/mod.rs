use std::sync::Arc;

use uuid::Uuid;

use crate::{
    api::alert::AlertData,
    strategy_manager::{trade_error::TradeError},
    App,
};

pub mod alpaca_client;
pub mod order;

pub enum Broker {
    Alpaca,
}

pub trait TradeManager {
    fn get_account(&self) -> Result<(), ()>;
}

pub struct Account {
    id: Uuid,
}

pub struct TradeExecutor;

impl TradeExecutor {
    // pub fn new(app: Arc<App>) -> Self {
    //     Self { app }
    // }

    pub async fn execute_trade<T>(
        &self,
        broker: Broker,
    ) -> Result<(), TradeError> {
        // let account = self.get_account();
        // let order = self.create_order(input);

        Ok(())
    }
}
