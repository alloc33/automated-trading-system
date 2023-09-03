use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TradeError {
    #[error("{0}")]
    AlertError(String),
}
