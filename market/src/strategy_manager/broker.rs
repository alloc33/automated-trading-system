use serde::Deserialize;

use super::{trade_error::TradeError, Order, TradingClient};
use crate::api::alert::AlertData;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
    // TODO: ?add more brokers
}
