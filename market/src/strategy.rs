use serde::Deserialize;
use uuid::Uuid;

use crate::strategy_manager::Broker;

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub broker: Broker,
    pub max_order_retries: u8,
    pub order_retry_delay: f64,
}
