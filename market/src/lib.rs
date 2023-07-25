pub mod auth;
pub mod config;
pub mod error;
pub mod handler;
pub mod model;
use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    body,
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request, StatusCode},
    middleware,
    middleware::{from_fn, from_fn_with_state},
    response::{Html, Response},
    routing::{get, post},
    Extension, Router,
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

// use std::{sync::Arc, time::Duration};
// use axum::{
//     middleware,
//     routing::{get, post},
//     Router,
// };
// use sqlx::{Pool, Postgres};
// use config::Config;

pub struct App {
    db: PgPool,
    config: AppConfig,
}

use crate::{
    auth::auth,
    handler::{another_handler, check_health},
};

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
        .route("/", get(check_health))
        .route("/another", get(another_handler))
        .layer(from_fn_with_state(app_state.clone(), auth))
        .layer(Extension(app_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
