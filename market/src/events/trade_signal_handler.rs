use super::{EventHandler, HandleEventError, TradingSignal};
use crate::trade_executor::TradeExecutor;

pub struct TradeSignalHandler {
    trade_executor: TradeExecutor,
}

impl TradeSignalHandler {
    pub fn new(trade_executor: TradeExecutor) -> Self {
        Self { trade_executor }
    }
}

impl EventHandler for TradeSignalHandler {
    type EventPayload = TradingSignal;

    fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
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
