pub mod alert;
pub mod error;
pub mod pagination;

use axum::{http::StatusCode, Json};

use self::error::ApiError;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
