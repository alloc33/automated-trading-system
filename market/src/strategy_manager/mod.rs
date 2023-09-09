pub mod macd_ema_v0;
pub mod trade_error;

use crate::{
    api::alert::AlertData,
    events::{EventHandler, HandleEventError},
    trade_executor::{Broker, TradeExecutor},
};

pub struct StrategyManager {
    trade_executor: TradeExecutor,
}

impl StrategyManager {
    pub fn new(trade_executor: TradeExecutor) -> Self {
        Self { trade_executor }
    }
}

#[axum::async_trait]
impl EventHandler for StrategyManager {
    type EventPayload = AlertData;

    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
        match event.exchange.as_str() {
            "BATS" | "NYSE" | "NASDAQ" => self
                .trade_executor
                .execute_trade(event, Broker::Alpaca)
                .await
                .map_err(HandleEventError::TradeError),

            _ => Err(HandleEventError::UnknownExchange("".to_string())),
        }
    }
}
