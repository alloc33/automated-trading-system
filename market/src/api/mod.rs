pub mod error;
pub mod pagination;
pub mod price;
pub mod webhook_receiver;
pub mod alert;

use axum::{http::StatusCode, Json};

use self::error::ApiError;

pub type Response<T> = Result<(StatusCode, Json<T>), ApiError>;
