pub mod error;
pub mod pagination;
pub mod trading_alert;

use axum::{http::StatusCode, Json};

use self::error::ApiError;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
