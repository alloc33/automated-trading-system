pub mod alert;
pub mod error;
pub mod pagination;
pub mod price;
pub mod strategy;
pub mod handlers;

use axum::{http::StatusCode, Json};

use self::error::ApiError;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
