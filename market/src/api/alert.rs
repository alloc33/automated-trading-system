use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use super::{error::ApiError, price::Price};
use crate::{app_config::AppConfig, clients::BrokerClient, strategy::Strategy};

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
pub struct WebhookAlertData {
    pub strategy_id: Uuid,
    pub ticker: String,
    pub timeframe: String,
    pub exchange: String,
    pub signal_data: SignalData,
    pub bar_data: BarData,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// Signal type to receive from TradingView.
/// Take profits are being calculated on the server side.
pub enum SignalType {
    OpenLong(TrailStopPrice),
    OpenShort(TrailStopPrice),
    StopLossUpdate(TrailStopPrice),
}

#[derive(Debug, Clone, Serialize)]
pub struct TrailStopPrice(pub Decimal);

impl<'de> Deserialize<'de> for TrailStopPrice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let decimal = Decimal::deserialize(deserializer)?;
        Ok(TrailStopPrice(decimal))
    }
}

impl<'de> Deserialize<'de> for SignalType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SignalTypeWrapper {
            #[serde(rename = "signal_type")]
            signal_type: String,
            trail_stop_price: TrailStopPrice,
        }

        let value = SignalTypeWrapper::deserialize(deserializer)?;

        match value.signal_type.as_str() {
            "open_long" => Ok(SignalType::OpenLong(value.trail_stop_price)),
            "open_short" => Ok(SignalType::OpenShort(value.trail_stop_price)),
            "stop_loss_update" => Ok(SignalType::StopLossUpdate(value.trail_stop_price)),
            _ => Err(serde::de::Error::unknown_variant(
                &value.signal_type,
                &["open_long", "open_short", "stop_loss_update"],
            )),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignalData {
    pub signal_type: SignalType,
    pub trail_stop_price: Option<Decimal>,
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
