use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    pagination::{Pagination, PaginationQuery},
    Response,
};
use crate::App;

#[derive(Debug, Deserialize)]
pub struct NewTradingAlert {
    pub ticker: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradingAlert {
    pub id: Uuid,
    pub ticker: String,
    pub created_at: DateTime<Utc>,
}

pub async fn process_trading_alert(
    State(app): State<Arc<App>>,
    Json(body): Json<NewTradingAlert>,
) -> Response<TradingAlert> {
    let row = sqlx::query!(
        "
        INSERT INTO trading_alerts (
            trading_alert_id,
            ticker,
            created_at,
            modified_at
        )
        VALUES ($1, $2, NOW(), NOW())
        RETURNING trading_alert_id, ticker, created_at
        ",
        uuid7::new_v7(),
        body.ticker.clone()
    )
    .fetch_one(&app.db)
    .await?;

    let trading_alert = TradingAlert {
        id: row.trading_alert_id,
        ticker: row.ticker,
        created_at: row.created_at,
    };

    Ok((StatusCode::CREATED, Json(trading_alert)))
}

pub async fn get_trading_alerts(
    State(app): State<Arc<App>>,
    Query(pagination): Query<PaginationQuery>,
) -> Response<Pagination<TradingAlert>> {
    let total = sqlx::query!("SELECT COUNT(*) as total FROM trading_alerts")
        .fetch_one(&app.db)
        .await?
        .total
        .unwrap_or_default();

    let results = sqlx::query_as!(
        TradingAlert,
        "
        SELECT 
            trading_alert_id as id,
            ticker,
            created_at
        FROM trading_alerts
        ORDER BY created_at DESC
        LIMIT $1
        OFFSET $2
        ",
        pagination.limit,
        pagination.offset
    )
    .fetch_all(&app.db)
    .await?;

    Ok((
        StatusCode::OK,
        Json(Pagination::new(results, total, pagination)),
    ))
}
