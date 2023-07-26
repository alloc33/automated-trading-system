use std::sync::Arc;

use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::info;

use super::error::ApiError;
use crate::App;

#[derive(Debug, Clone, Deserialize)]
pub struct TradingAlert {
    pub date_time: DateTime<Utc>,
}

pub async fn process_trading_alert(
    State(app): State<Arc<App>>,
    Json(body): Json<TradingAlert>,
) -> Result<(), ApiError> {
    info!("Trading alert received: {body:#?}");

    info!("Processing trading alert...");

    Ok(())
}

pub async fn get_trading_alerts(State(app): State<Arc<App>>) -> Result<(), ApiError> {
    info!("Getting trading alerts...");
    Ok(())
}
