pub mod error;
pub mod trading_alert;

use axum::Json;
use error::ApiError;

pub(crate) type Response<T> = Result<Json<T>, ApiError>;
