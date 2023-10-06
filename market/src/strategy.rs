use serde::Deserialize;
use uuid::Uuid;

use crate::objects::Broker;

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub broker: Broker,
    pub currency_type: CurrencyType,
    pub max_order_retries: u8,
    pub order_retry_delay: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyType {
    Crypto,
    Stock,
}
