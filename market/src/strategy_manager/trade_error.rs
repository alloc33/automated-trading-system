use thiserror::Error as ThisError;

use super::Order;

#[derive(Debug, ThisError)]
pub enum TradeError {
    #[error("{0}")]
    InsufficientFunds(String),
    #[error("Order max retries reached. {0}")]
    MaxRetriesReached(Order),
}
