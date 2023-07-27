pub mod api;
pub mod auth;
pub mod config;
pub mod model;

use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use api::{
    error::{self, ApiError},
    trading_alert::{get_trading_alerts, process_trading_alert},
};
use auth::auth;
// use load_shed
use axum::{
    body::Body,
    http::{request::Request, response::Response, Method, Uri},
    middleware::{from_fn_with_state, Next},
    routing::post,
    Extension,
};
use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, Path, State},
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Router,
};
use config::AppConfig;
use sqlx::{postgres::PgConnectOptions, Error as SqlxError, PgPool, Pool, Postgres};
use thiserror::Error as ThisError;
use tower::{BoxError, ServiceBuilder};
use tower_http::{self, trace::TraceLayer};
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct App {
    db: PgPool,
    config: AppConfig,
}

pub async fn build_state(config: AppConfig) -> Result<App, SqlxError> {
    let opts = config.database_url.parse::<PgConnectOptions>()?;

    let pool = sqlx::pool::PoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(None)
        .min_connections(1)
        .connect_with(opts)
        .await?;

    let app = App { db: pool, config };
    Ok(app)
}

pub fn build_routes(app_state: Arc<App>) -> Router {
    Router::new()
        .route("/alert", post(process_trading_alert))
        .route("/alert", get(get_trading_alerts))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(app_state.clone(), auth))
                // .layer(HandleErrorLayer::new(handle_error))
                // .load_shed()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            tower_http::trace::DefaultMakeSpan::new().level(Level::DEBUG),
                        )
                        .on_response(
                            tower_http::trace::DefaultOnResponse::new().level(Level::DEBUG),
                        ),
                ), /* .layer(
                    *     tower_http::trace::DefaultMakeSpan::new().level(Level::DEBUG),
                    *     tower_http::trace::DefaultOnResponse::new().level(Level::DEBUG),
                    * ) */
        )
        // .with_state(app_state)
        .with_state(Arc::clone(&app_state))
}

// async fn custom_error_handler(error: ApiError) -> (StatusCode, serde_json::Value) {
//     // Handle ApiError
//     // if let Some(api_error) = error.downcast_ref::<ApiError>() {
//     // Return the appropriate status code and error message
//     let status_code = error.http_status();
//     let error_message = error.to_string();
//     return (status_code, serde_json::json!({ "error": error_message }));
//     // }

//     // Handle other errors or fallback to generic internal server error
//     // (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": "Internal server error" }))
// }

// async fn handle_error(error: ApiError) -> impl IntoResponse {
//     // if error.is::<tower::timeout::error::Elapsed>() {
//     //     return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
//     // }

//     // if error.is::<tower::load_shed::error::Overloaded>() {
//     //     return (
//     //         StatusCode::SERVICE_UNAVAILABLE,
//     //         Cow::from("service is overloaded, try again later"),
//     //     );
//     // }

//     // if error.is::<ApiError>() {
//     //     return (StatusCode::UNAUTHORIZED, Cow::from("jblablabla"));
//     // }

//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         Cow::from(format!("Unhandled internal error: {}", error)),
//     )
// }

async fn handle_error(error: BoxError) -> impl IntoResponse {
    // if error.is::<tower::timeout::error::Elapsed>() {
    //     // Handle ApiError
    //     let status_code = api_error.http_status();
    //     let error_message = api_error.to_string();

    //     return (status_code, error_message);
    // }
    //
    //
    println!("{error:#?}");
    println!("somehting");

    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
