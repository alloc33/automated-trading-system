use std::fmt::Display;

use axum::{
    extract::rejection::JsonRejection,
    http::{Request, StatusCode},
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use thiserror::Error as ThisError;
use tracing::error;

use crate::clients::BrokerClientError;

pub const INTERNAL_SERVER_ERROR: &str = "Internal server error occurred...";
pub const PAYLOAD_TOO_LARGE: &str = "Request payload too large...";
pub const DATABASE_UNAVAILABLE: &str = "Database is unavailable...";

#[derive(Clone, Debug, Serialize, ThisError)]
pub enum ConstraintError {
    Unknown(String),
    Null,
    // Other(String),
}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Unknown(constraint) => constraint,
            Self::Null => "Non-null contraint",
        };
        write!(f, "{}", msg)
    }
}

/// `OperationError` describes possible errors of API operations.
#[derive(Debug, ThisError)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error(transparent)]
    ConstraintError(#[from] ConstraintError),

    /// IO related error.
    ///
    /// HTTP status code 400
    #[error("{0}")]
    IOError(String),

    /// Payload too large error.
    ///
    /// HTTP status code 413
    #[error("Request payload too large...")]
    PayloadTooLarge,

    /// Unauthorized error.
    ///
    /// HTTP status code 401
    #[error("{0}")]
    Unauthorized(String), // Added Unauthorized variant

    /// Failed to deserialize json.
    ///
    /// HTTP status code 422
    ///
    /// Relates to:
    ///     required fields /
    ///     enum variants /
    ///     invalid values for string formats (
    ///         uuid /
    ///         decimal
    ///     )

    /// Unexpected internal server error.
    ///
    /// HTTP status code 500
    #[error("Internal server error occurred...")]
    InternalServerError,

    /// Service unavailable error.
    ///
    /// HTTP status code 503
    #[error("Database is unavailable...")]
    ServiceUnavailable,

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),

    #[error(transparent)]
    TradingClientError(#[from] BrokerClientError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::IOError(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::JsonExtractorRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection.to_string())
            }
            Self::PayloadTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, PAYLOAD_TOO_LARGE.to_owned()),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                INTERNAL_SERVER_ERROR.to_owned(),
            ),
            Self::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                DATABASE_UNAVAILABLE.to_owned(),
            ),
            Self::ConstraintError(err) => (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),

            // Apca crate doesn't allow to get status code from it's response and deserialize error
            // message properly. We get error message as debug string of entire result and
            // hardcoded status code. Real status code should be shown in debug message.
            Self::TradingClientError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(serde_json::json!({
            "Error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<StatusCode> for ApiError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::BAD_REQUEST => Self::BadRequest("Bad request".to_owned()),
            StatusCode::NOT_FOUND => Self::NotFound("Not found".to_owned()),
            StatusCode::PAYLOAD_TOO_LARGE => Self::PayloadTooLarge,
            StatusCode::INTERNAL_SERVER_ERROR => Self::InternalServerError,
            StatusCode::SERVICE_UNAVAILABLE => Self::ServiceUnavailable,
            StatusCode::UNAUTHORIZED => Self::Unauthorized("Unauthorized".to_owned()),
            _ => Self::InternalServerError,
        }
    }
}

// Convenience methods and constructors of particular types of error
impl ApiError {
    #[must_use]
    pub fn http_status(&self) -> StatusCode {
        match self {
            Self::JsonExtractorRejection(_)
            | Self::IOError(_)
            | Self::BadRequest(_)
            | Self::NotFound(_) => StatusCode::BAD_REQUEST,
            Self::ConstraintError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::TradingClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn internal_error<E>(err: E) -> Self
    where
        E: Display,
    {
        error!("{err}");
        Self::InternalServerError
    }
}

impl<T> From<Request<T>> for ApiError {
    fn from(_: Request<T>) -> Self {
        Self::InternalServerError
    }
}

impl TryFrom<sqlx::Error> for ConstraintError {
    type Error = sqlx::Error;

    fn try_from(value: sqlx::Error) -> Result<Self, Self::Error> {
        match value {
            sqlx::Error::Database(database_err) => {
                let constraint = database_err.constraint();
                Ok(match constraint {
                    Some(unknown) => ConstraintError::Unknown(unknown.to_owned()),
                    None => ConstraintError::Null,
                })
            }
            err => Err(err),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        error!("sqlx error: {}", err);
        match err {
            sqlx::Error::Io(_)
            | sqlx::Error::Tls(_)
            | sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed => Self::ServiceUnavailable,
            err @ sqlx::Error::Database(_) => match err.try_into() {
                Ok(contraint_error) => Self::ConstraintError(contraint_error),
                Err(_) => Self::ServiceUnavailable,
            },
            _ => Self::InternalServerError,
        }
    }
}
