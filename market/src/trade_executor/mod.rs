use std::sync::Arc;

use uuid::Uuid;

use crate::{
    api::alert::AlertData,
    strategy_manager::{trade_error::TradeError, Order},
    App
};

pub mod alpaca_client;
pub mod order;

pub trait TradeManager {
    fn get_account(&self) -> Result<(), ()>;
}

pub struct Account {
    id: Uuid,
}

pub struct TradeExecutor;

type TradeExecutorResult = Result<u64, TradeError>;

impl TradeExecutor {
    // pub fn new(app: Arc<App>) -> Self {
    //     Self { app }
    // }

    pub async fn execute_order(&self, order: &Order) -> TradeExecutorResult {
        // let account = self.get_account();
        // let order = self.create_order(input);

        Ok(chrono::Utc::now().timestamp() as u64)
    }
}
