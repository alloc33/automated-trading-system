pub mod alert;
pub mod error;
pub mod handlers;
pub mod objects;
pub mod pagination;
pub mod price;
pub mod strategy;

use axum::Json;

use self::error::ApiError;

pub(crate) type Response<T> = Result<Json<T>, ApiError>;
