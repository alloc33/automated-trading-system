pub mod trade_error;

use crate::{
    app_config::Strategy,
    events::{EventHandler, HandleEventError},
    trade_executor::{Broker, TradeExecutor},
};

/// Represents a trade signal received from the webhook.
pub struct TradeSignal {
    pub strategy: Strategy,
    pub exchange: String,
    // TODO: Add more fields
}

impl TradeSignal {
    pub fn new(strategy: Strategy, exchange: String) -> Self {
        Self {
            strategy,
            exchange,
        }
    }
}

pub struct StrategyManager {
    trade_executor: TradeExecutor,
}

impl StrategyManager {
    pub fn new(trade_executor: TradeExecutor) -> Self {
        Self {
            trade_executor,
        }
    }
}

#[axum::async_trait]
impl EventHandler for StrategyManager {
    type EventPayload = TradeSignal;

    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
        match event.exchange.as_str() {
            "BATS" | "NYSE" | "NASDAQ" => self
                .trade_executor
                .execute_trade(event, Broker::Alpaca)
                .await
                .map_err(HandleEventError::TradeError),

            _ => Err(HandleEventError::UnknownExchange("".to_string())),
        }
        Ok(())
    }
}
