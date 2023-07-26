pub mod api;
pub mod auth;
pub mod config;
pub mod model;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use api::{
    error,
    trading_alert::{get_trading_alerts, process_trading_alert},
};
use auth::auth;
use axum::{
    body::{Body, Bytes},
    error_handling::HandleErrorLayer,
    http::{request::Request, response::Response, Method, StatusCode, Uri},
    middleware::{from_fn_with_state, Next},
    routing::{get, post},
    BoxError, Extension, Router,
};
use config::AppConfig;
use sqlx::{postgres::PgConnectOptions, Error as SqlxError, PgPool, Pool, Postgres};
use thiserror::Error as ThisError;
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{self, TraceLayer},
};
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use load_shed

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
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(from_fn_with_state(app_state.clone(), auth))
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .with_state(app_state)
}

async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{} {}` failed with {}", method, uri, err),
    )
}
