pub mod error;
pub mod pagination;
pub mod trading_alert;

use axum::{http::StatusCode, Json};

use self::error::ApiError;
use serde::Serialize;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
