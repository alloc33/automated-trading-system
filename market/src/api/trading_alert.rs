use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::uuid;

use super::{error::ApiError, Response};
use crate::App;

#[derive(Debug, Deserialize)]
pub struct NewTradingAlert {
    pub ticker: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradingAlert {
    pub ticker: String,
}

pub async fn process_trading_alert(
    State(app): State<Arc<App>>,
    Json(body): Json<NewTradingAlert>,
) -> Response<TradingAlert> {
    let mock_alrt = TradingAlert {
        ticker: body.ticker,
    };

    Ok((StatusCode::CREATED, Json(mock_alrt)))
}

// pub async fn get_trading_alerts(State(app): State<Arc<App>>) -> Result<Response, ApiError> {
//     // Retrieve the trading alerts here...

//     // For the purpose of this example, let's assume we retrieved some trading alerts as a list
// of     // strings. You can replace the following line with your actual logic to retrieve the
//     // alerts.
//     let alerts: Vec<String> = vec!["Alert 1".to_string(), "Alert 2".to_string()];

//     // Create a mock response body containing the trading alerts.
//     let response_body = ApiResponseBody {
//         message: "Trading alerts retrieved.".to_string(),
//         body: None,
//     };

//     // Return the ApiResponse with the mock response body and a status code of OK (200).
//     Ok((StatusCode::OK, Json(response_body)))
// }
