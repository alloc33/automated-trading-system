use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::Json};
use serde::{Serialize, Serializer};
use serde_json::error::Category;
use thiserror::Error as ThisError;
use tracing::error;

pub const INTERNAL_SERVER_ERROR: &str = "Internal server error occurred...";
pub const PAYLOAD_TOO_LARGE: &str = "Request payload too large...";
pub const DATABASE_UNAVAILABLE: &str = "Database is unavailable...";

async fn error_handler(error: ApiError) -> Json<ErrorResponse> {
    let error_response = ErrorResponse {
        code: error.http_status().as_u16(),
        error: error.to_string(),
    };

    Json(error_response)
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
}

#[derive(Clone, Debug, Serialize, ThisError)]
pub enum ConstraintError {
    // UniqueUserEmail,
    // UniqueFeatureUuid,
    // UniqueFeatureInPlanAsExtra,
    // UniqueFeatureDependency,
    // UniquePriceForCurrency,
    // UniqueCountryRegion,
    // UserDoesNotExist,
    // OrganizationDoesNotExist,
    // UserAlreadyExistsInOrganization,
    // ParentOrganizationDoesNotExist,
    // PlanDoesNotExist,
    // FeatureDoesNotExist,
    // OpportunityDoesNotExist,
    // NotNullPlanOrFeature,
    Unknown(String),
    Null,
    // Other(String),
}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            // Self::Other(msg) => msg,
            // Self::UniqueUserEmail => "Email already in use",
            // Self::UniqueFeatureUuid => "Feature with this UUID already exists",
            // Self::UniqueFeatureInPlanAsExtra => "Feature already exists in plan as extra",
            // Self::UniqueFeatureDependency => "Feature dependency already exists",
            // Self::UniquePriceForCurrency => "Price for currency already exists",
            // Self::UniqueCountryRegion => "Country region already exists",
            // Self::UserDoesNotExist => "User does not exist",
            // Self::OrganizationDoesNotExist => "Organization does not exist",
            // Self::UserAlreadyExistsInOrganization => "User already exists",
            // Self::ParentOrganizationDoesNotExist => "Parent organization does not exist",
            // Self::PlanDoesNotExist => "Plan does not exist",
            // Self::FeatureDoesNotExist => "Feature does not exist",
            // Self::OpportunityDoesNotExist => "Opportunity does not exist",
            // Self::NotNullPlanOrFeature => "Plan or feature cannot be null on price",
            Self::Unknown(constraint) => constraint,
            Self::Null => "Non-null contraint",
        };
        write!(f, "{}", msg)
    }
}

/// `OperationError` describes possible errors of API operations.
#[derive(Debug, Serialize, ThisError)]
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

    /// Json deserialization errors.
    ///
    /// HTTP status code 400
    #[error("{0}")]
    JsonParseError(String),

    /// Payload too large error.
    ///
    /// HTTP status code 413
    #[error(transparent)]
    PayloadTooLarge(#[from] PayloadTooLarge),

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
    #[error(transparent)]
    InternalServerError(#[from] InternalServerError),

    /// Service unavailable error.
    ///
    /// HTTP status code 503
    #[error(transparent)]
    ServiceUnavailable(#[from] ServiceUnavailable),
}

// Convenience methods and constructors of particular types of error
impl ApiError {
    #[must_use]
    pub fn http_status(&self) -> StatusCode {
        match self {
            Self::JsonParseError(_)
            | Self::IOError(_)
            | Self::BadRequest(_)
            | Self::NotFound(_) => StatusCode::BAD_REQUEST,
            Self::ConstraintError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
    pub fn internal_error<E>(err: E) -> Self
    where
        E: Display,
    {
        error!("{err}");
        Self::InternalServerError(InternalServerError)
    }
}



impl From<axum::Error> for ApiError {
    fn from(err: axum::Error) -> Self {
        match err.source() {
            Some(inner_error) => {
                // Check if the inner error is ApiError, in which case, return it directly
                if let Some(api_error) = inner_error.downcast_ref::<ApiError>() {
                    return *api_error;
                }
            }
            None => {} // No inner error, continue
        }

        // Default case: Convert to InternalServerError
        ApiError::InternalServerError(InternalServerError)
    }
}

impl TryFrom<sqlx::Error> for ConstraintError {
    type Error = sqlx::Error;

    fn try_from(value: sqlx::Error) -> Result<Self, Self::Error> {
        match value {
            sqlx::Error::Database(database_err) => {
                let constraint = database_err.constraint();
                Ok(match constraint {
                    // Some("unique_organization_user") => {
                    //     ConstraintError::UserAlreadyExistsInOrganization
                    // }
                    // Some("unique_feature_uuid") => ConstraintError::UniqueFeatureUuid,
                    // Some("users_unique_email") => ConstraintError::UniqueUserEmail,
                    // Some("unique_feature_dependency") =>
                    // ConstraintError::UniqueFeatureDependency,
                    // Some("unique_price_for_currency") => ConstraintError::UniquePriceForCurrency,
                    // Some("unique_country_region") => ConstraintError::UniqueCountryRegion,
                    // Some("fk_link_user" | "fk_order_user") => ConstraintError::UserDoesNotExist,
                    // Some("fk_link_organization" | "fk_order_organization" | "fk_contract_org") =>
                    // {     ConstraintError::OrganizationDoesNotExist
                    // }
                    // Some("fk_parent_org") => ConstraintError::ParentOrganizationDoesNotExist,
                    // Some("fk_feature_plan" | "fk_price_plan" | "fk_order_plan") => {
                    //     ConstraintError::PlanDoesNotExist
                    // }
                    // Some(
                    //     "fk_plan_feature"
                    //     | "fk_dependency_feature"
                    //     | "fk_feature_dependency"
                    //     | "fk_price_feature",
                    // ) => ConstraintError::FeatureDoesNotExist,
                    // Some("fk_opportunity_org") => ConstraintError::OrganizationDoesNotExist,
                    // Some("fk_quote_opportunity") => ConstraintError::OpportunityDoesNotExist,
                    // Some("not_null_plan_or_feature") => ConstraintError::NotNullPlanOrFeature,
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
            | sqlx::Error::PoolClosed => Self::ServiceUnavailable(ServiceUnavailable::database()),
            err @ sqlx::Error::Database(_) => match err.try_into() {
                Ok(contraint_error) => Self::ConstraintError(contraint_error),
                Err(_) => Self::ServiceUnavailable(ServiceUnavailable::database()),
            },
            _ => Self::InternalServerError(InternalServerError),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        if matches!(err.classify(), Category::Data) {
            ApiError::BadRequest(err.to_string())
        } else {
            ApiError::JsonParseError(err.to_string())
        }
    }
}

/// Request body is too large.
///
/// HTTP status code 413
#[derive(Copy, Clone, Debug)]
pub struct PayloadTooLarge;

impl Error for PayloadTooLarge {}

impl Display for PayloadTooLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PAYLOAD_TOO_LARGE)
    }
}

impl Serialize for PayloadTooLarge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(PAYLOAD_TOO_LARGE)
    }
}

/// Unexpected internal server error.
///
/// HTTP status code 500
#[derive(Copy, Clone, Debug)]
pub struct InternalServerError;

impl Error for InternalServerError {}

impl Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", INTERNAL_SERVER_ERROR)
    }
}

impl Serialize for InternalServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(INTERNAL_SERVER_ERROR)
    }
}

/// Service unavailable error.
///
/// HTTP status code 503
#[derive(Copy, Clone, Debug, Serialize, ThisError)]
pub enum ServiceUnavailable {
    #[error(transparent)]
    Database(DatabaseUnavailable),
}

impl ServiceUnavailable {
    #[must_use]
    pub fn database() -> Self {
        Self::Database(DatabaseUnavailable)
    }
}

/// Variant for [`ServiceUnavailable`] error
/// which represents that the database cannot be reached.
///
/// Returned when database is unavailable due network/TLS related issues.
#[derive(Copy, Clone, Debug)]
pub struct DatabaseUnavailable;

impl Error for DatabaseUnavailable {}

impl Display for DatabaseUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DATABASE_UNAVAILABLE)
    }
}

impl Serialize for DatabaseUnavailable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(DATABASE_UNAVAILABLE)
    }
}
