use serde::Deserialize;

use crate::api::alert::AlertData;

use super::{TradingClient, Order, trade_error::TradeError};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
    // TODO: ?add more brokers
}
