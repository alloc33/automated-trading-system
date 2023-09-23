pub mod alert;
pub mod error;
pub mod handlers;
pub mod objects;
pub mod pagination;
pub mod price;
pub mod strategy;

use axum::{http::StatusCode, Json};

use self::error::ApiError;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
