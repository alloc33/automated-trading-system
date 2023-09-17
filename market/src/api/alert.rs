use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use super::{error::ApiError, price::Price};
use crate::{app_config::AppConfig, strategy::Strategy};

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

#[derive(Debug, Clone)]
pub struct TradeSignal {
    pub strategy: Strategy,
    pub ticker: String,
    pub timeframe: String,
    pub exchange: String,
    pub alert_type: AlertType,
    pub bar: BarData,
    pub time: DateTime<Utc>,
}

impl TradeSignal {
    pub fn from_alert_data(
        alert_data: WebhookAlertData,
        config: &AppConfig,
    ) -> Result<Self, ApiError> {
        let strategy_id = alert_data.strategy_id;

        let validated_strategy = config
            .strategies
            .iter()
            .find(|strategy| strategy.id == strategy_id)
            .ok_or_else(|| {
                let msg = format!("Unknown strategy - {}", strategy_id);
                tracing::error!(msg);
                ApiError::BadRequest(msg)
            })?;

        if !validated_strategy.enabled {
            let msg = format!(
                "Strategy {} with id {} is disabled",
                validated_strategy.name, validated_strategy.id
            );
            tracing::error!(msg);
            return Err(ApiError::BadRequest(msg));
        }

        Ok(Self {
            strategy: validated_strategy.clone(),
            ticker: alert_data.ticker,
            timeframe: alert_data.timeframe,
            exchange: alert_data.exchange,
            alert_type: alert_data.alert_type,
            bar: alert_data.bar,
            time: alert_data.time,
        })
    }
}
