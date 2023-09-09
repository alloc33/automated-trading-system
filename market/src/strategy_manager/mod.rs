pub mod trade_error;

use std::sync::Arc;

use uuid::Uuid;
use crate::{
    api::alert::AlertData,
    app_config::Strategy,
    events::{EventHandler, HandleEventError},
    trade_executor::TradeExecutor,
    App,
};

pub struct StrategyManager {
    app_state: Arc<App>,
    trade_executor: TradeExecutor,
}

pub struct Order {}

impl StrategyManager {
    pub fn new(app_state: Arc<App>, trade_executor: TradeExecutor) -> Self {
        Self {
            app_state,
            trade_executor,
        }
    }

    fn find_strategy(&self, strategy_id: Uuid) -> Option<&Strategy> {
        self.app_state
            .config
            .strategies
            .iter()
            .find(|strategy| strategy.id == strategy_id)
    }
}

#[axum::async_trait]
impl EventHandler for StrategyManager {
    type EventPayload = AlertData;

    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
        if let Some(strategy) = self.find_strategy(event.strategy_id) {
            // self.trade_executor
            // .execute_trade(event, strategy.broker)
            // .await
            // .map_err(HandleEventError::TradeError)
            Ok(())
        } else {
            Err(HandleEventError::UnknownStrategy(event.strategy_id.to_string()))
        }
        // match event.exchange.as_str() {
        //     "BATS" | "NYSE" | "NASDAQ" => self
        //         .trade_executor
        //         .execute_trade(event, Broker::Alpaca)
        //         .await
        //         .map_err(HandleEventError::TradeError),

        //     _ => Err(HandleEventError::UnknownExchange("".to_string())),
        // };
    }
}
