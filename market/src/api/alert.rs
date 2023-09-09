use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

use super::price::Price;

// NOTE: Webhook body example:
// {
// 	"webhook_key": "d48ab5eec650c0351930f758e712d17f1cd829c603718c6ceb76869a3648be0b",
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
    pub webhook_key: String,
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
