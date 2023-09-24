use serde::Deserialize;
use uuid::Uuid;

use crate::strategy_manager::Exchange;

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub exchange: Exchange,
    pub max_order_retries: u8,
    pub order_retry_delay: f64,
}
