pub mod macd_ema_v0;
pub mod trade_error;

use async_trait::async_trait;

use crate::{
    events::{EventHandler, HandleEventError, TradingSignal},
    trade_executor::TradeExecutor,
};

pub struct StrategyManager {
    trade_executor: TradeExecutor,
}

impl StrategyManager {
    pub fn new(trade_executor: TradeExecutor) -> Self {
        Self { trade_executor }
    }
}

// TODO: Retry signal with trade executor
#[async_trait]
impl EventHandler for StrategyManager {
    type EventPayload = TradingSignal;

    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
        match event {
            TradingSignal::Long => {
                // Handle the Long signal event here
                Ok(())
            }
            TradingSignal::Short => {
                // Handle the Short signal event here
                Ok(())
            }
            TradingSignal::StopLoss => {
                // Handle the StopLoss signal event here
                Ok(())
            }
            TradingSignal::TakeProfit => {
                // Handle the TakeProfit signal event here
                Ok(())
            }
        }
    }
}
