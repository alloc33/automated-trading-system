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

use axum::http::StatusCode;

pub async fn process_trading_alert(
    State(app): State<Arc<App>>,
    Json(body): Json<TradingAlert>,
) -> Result<(), ApiError> {
    info!("Trading alert received: {body:#?}");

    info!("Processing trading alert...");

    Ok(())
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TestErr {
    pub test_1: String,
    pub test_2: String,
}

pub async fn get_trading_alerts(State(app): State<Arc<App>>) -> Result<Json<String>, ApiError> {
    info!("Getting trading alerts...");
    let is_alert_nice = false;

    // if !is_alert_nice {
    //     return Err(ApiError::BadRequest("It's bad".to_string()))
    // }

    Err(ApiError::BadRequest("It's bad".to_string()))
    // Ok(Json("nice alert!".to_string()))
}
