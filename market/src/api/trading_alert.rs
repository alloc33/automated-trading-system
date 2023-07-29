use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use super::{
    pagination::{Pagination, PaginationQuery},
    Response,
};
use crate::App;

#[derive(Debug, Deserialize)]
pub struct NewTradingAlert {
    pub ticker: String,
    pub exchange: String,
    pub alert_type: AlertType,
    pub bar_data: BarData,
    pub alert_fire_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum AlertType {
    Long,
    Short,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarData {
    bar_time: DateTime<Utc>,
    bar_open: Decimal,
    bar_high: Decimal,
    bar_low: Decimal,
    bar_close: Decimal,
    bar_volume: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradingAlert {
    pub id: Uuid,
    pub ticker: String,
    pub exchange: String,
    pub alert_type: AlertType,
    pub bar_data: BarData,
    pub alert_fire_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub async fn process_trading_alert(
    State(app): State<Arc<App>>,
    Json(alert): Json<NewTradingAlert>,
) -> Response<TradingAlert> {
    let row = sqlx::query!(
        "
        INSERT INTO trading_alerts (
            trading_alert_id,
            ticker,
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
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW()
        )
        RETURNING *
        ",
        uuid7::new_v7(),
        alert.ticker,
        alert.exchange,
        alert.alert_type.as_ref(),
        alert.bar_data.bar_time,
        alert.bar_data.bar_open,
        alert.bar_data.bar_high,
        alert.bar_data.bar_low,
        alert.bar_data.bar_close,
        alert.bar_data.bar_volume,
        alert.alert_fire_time
    )
    .fetch_one(&app.db)
    .await?;

    let trading_alert = TradingAlert {
        id: row.trading_alert_id,
        ticker: row.ticker,
        exchange: row.exchange,
        alert_type: AlertType::from_str(&row.alert_type).expect("Invalid alert type"),
        bar_data: BarData {
            bar_time: row.bar_time,
            bar_open: row.bar_open,
            bar_high: row.bar_high,
            bar_low: row.bar_low,
            bar_close: row.bar_close,
            bar_volume: row.bar_volume,
        },
        alert_fire_time: row.alert_fire_time,
        created_at: row.created_at,
    };

    Ok((StatusCode::CREATED, Json(trading_alert)))
}

pub async fn get_trading_alerts(
    State(app): State<Arc<App>>,
    Query(pagination): Query<PaginationQuery>,
) -> Response<Pagination<TradingAlert>> {
    // let total = sqlx::query!("SELECT COUNT(*) as total FROM trading_alerts")
    //     .fetch_one(&app.db)
    //     .await?
    //     .total
    //     .unwrap_or_default();

    // let results = sqlx::query!(
    //     "
    //     SELECT 
    //         trading_alert_id as id,
    //         ticker,
    //         exchange, 
    //         alert_type,
    //         bar_time,
    //         bar_open,
    //         bar_high,
    //         bar_low,
    //         bar_close,
    //         bar_volume,
    //         alert_fire_time,
    //         created_at
    //     FROM trading_alerts
    //     ORDER BY created_at DESC
    //     LIMIT $1
    //     OFFSET $2
    //     ",
    //     pagination.limit,
    //     pagination.offset
    // )
    // .fetch_all(&app.db)
    // .await?
    // .iter()
    // .map(|row| TradingAlert {
    //     id: row.id,
    //     ticker: row.ticker.clone(),
    //     exchange: row.exchange.clone(),
    //     alert_type: AlertType::from_str(&row.alert_type).expect("Invalid alert type"),
    //     bar_data: BarData {
    //         bar_time: row.bar_time,
    //         bar_open: row.bar_open,
    //         bar_high: row.bar_high,
    //         bar_low: row.bar_low,
    //         bar_close: row.bar_close,
    //         bar_volume: row.bar_volume,
    //     },
    //     alert_fire_time: row.alert_fire_time,
    //     created_at: row.created_at,
    // })
    // .collect::<Vec<_>>();

    Ok((
        StatusCode::OK,
        Json(Pagination::new(vec![], 0, pagination)),
    ))
}
