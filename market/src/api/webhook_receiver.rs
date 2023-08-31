use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::WithRejection;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use strum_macros::{AsRefStr, EnumString};

use super::{error::ApiError, Response};
use crate::{objects::Price, App};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAlert {
    pub webhook_key: String,
    pub ticker: String,
    pub timeframe: String,
    pub exchange: String,
    #[serde(rename = "type")]
    pub alert_type: AlertType,
    pub bar: BarData,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    Long,
    Short,
    StopLoss,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarData {
    pub time: DateTime<Utc>,
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Decimal,
}

pub async fn receive_alert(
    State(app): State<Arc<App>>,
    WithRejection(alert, _): WithRejection<Json<NewAlert>, ApiError>,
) -> Response<()> {
    if !is_valid_webhook_key(&alert.webhook_key) {
        return Err(ApiError::Unauthorized(
            "Webhook key is not correct or doesn't exist".to_string(),
        ));
    }

    _ = sqlx::query!(
        r#"
        INSERT INTO alerts (
            alert_id,
            ticker,
            timeframe,
            exchange, 
            alert_type,
            bar_time,
            bar_open,
            bar_high,
            bar_low,
            bar_close,
            bar_volume,
            alert_fire_time,
            created_at,
            modified_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW()
        )
        "#,
        uuid7::new_v7(),
        alert.ticker,
        alert.timeframe,
        alert.exchange,
        alert.alert_type.as_ref(),
        alert.bar.time,
        alert.bar.open.as_ref(),
        alert.bar.high.as_ref(),
        alert.bar.low.as_ref(),
        alert.bar.close.as_ref(),
        alert.bar.volume,
        alert.time
    )
    .execute(&app.db)
    .await?;

    Ok((StatusCode::CREATED, Json::default()))
}

fn is_valid_webhook_key(webhook_key: &str) -> bool {
    let env_webhook_key = std::env::var("WEBHOOK_KEY").unwrap_or_default();
    let hash = format!("{:x}", Sha256::digest(env_webhook_key.as_bytes()));
    hash == webhook_key
}
