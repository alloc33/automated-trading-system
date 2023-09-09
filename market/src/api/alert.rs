use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use super::price::Price;

// NOTE: Webhook body example:
// {
// 	"strategy_id": "C6557FC3-0D9A-447A-9D87-E417D98F2114",
// 	"time": "{{timenow}}",
// 	"exchange": "{{exchange}}",
// 	"ticker": "{{ticker}}",
// 	"type": "unknown",
// 	"bar": {
// 		"time": "{{time}}",
// 		"open": "{{open}}",
// 		"high": "{{high}}",
// 		"low": "{{low}}",
// 		"close": "{{close}}",
// 		"volume": "{{volume}}"
// 	}
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlertData {
    pub strategy_id: Uuid,
    pub ticker: String,
    pub timeframe: String,
    pub exchange: String,
    #[serde(rename = "type")]
    pub alert_type: AlertType,
    pub bar: BarData,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    Long,
    Short,
    StopLoss,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BarData {
    pub time: DateTime<Utc>,
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Decimal,
}
