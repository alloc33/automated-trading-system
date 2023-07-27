pub mod api;
pub mod auth;
pub mod config;
pub mod model;

use std::{sync::Arc, time::Duration};

use api::*;
use auth::auth;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use config::AppConfig;
use sqlx::{postgres::PgConnectOptions, Error as SqlxError, PgPool};
use tower::ServiceBuilder;
use tower_http::{self, trace::TraceLayer};
use tracing::Level;

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
        .route("/alert", post(api::trading_alert::process_trading_alert))
        // .route("/alert", get(api::trading_alert::get_trading_alerts))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(app_state.clone(), auth))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            tower_http::trace::DefaultMakeSpan::new().level(Level::INFO),
                        )
                        .on_response(
                            tower_http::trace::DefaultOnResponse::new().level(Level::INFO),
                        ),
                ),
        )
        .with_state(Arc::clone(&app_state))
}
