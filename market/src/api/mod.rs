pub mod alert;
pub mod webhook_receiver;
pub mod error;
pub mod pagination;
pub mod price;

use axum::{http::StatusCode, Json};

use self::error::ApiError;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
