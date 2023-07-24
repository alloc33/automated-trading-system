pub mod auth;
pub mod config;
pub mod error;
pub mod handler;
pub mod model;

use std::{sync::Arc, time::Duration};

use axum::{
    body,
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    middleware,
    middleware::{from_fn, from_fn_with_state},
    response::{Html, Response},
    routing::{get, post},
    Extension, Router,
};
use config::Config;
use sqlx::{postgres::PgConnectOptions, Error as SqlxError, Pool, Postgres};
use thiserror::Error as ThisError;
use tower::ServiceBuilder;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
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
    db: Pool<Postgres>,
    config: Config,
}

use crate::{
    auth::auth,
    handler::{another_handler, check_health},
};

pub async fn build_state() -> Result<App, SqlxError> {
    let config = Config::build();
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
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Router::new()
        .route("/", get(check_health))
        .route("/another", get(another_handler))
        .layer(
            ServiceBuilder::new()
                // add auth layer
                .layer(from_fn_with_state(app_state.clone(), auth))
                // .layer(
                //     TraceLayer::new_for_http()
                //         .make_span_with(|request: &Request<_>| {
                //             // Log the matched route's path (with placeholders not filled in).
                //             // Use request.uri() or OriginalUri if you want the real path.
                //             let matched_path = request
                //                 .extensions()
                //                 .get::<MatchedPath>()
                //                 .map(MatchedPath::as_str);
                //             info_span!(
                //                 "http_request",
                //                 method = ?request.method(),
                //                 matched_path,
                //                 some_other_field = tracing::field::Empty,
                //             )
                //         })
                //         .on_request(|_request: &Request<_>, _span: &Span| {
                //             // You can use `_span.record("some_other_field", value)` in one of
                // these             // closures to attach a value to the initially
                // empty field in the             // info_span created above.
                //         })
                //         .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                //             // ...
                //         })
                //         .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                //             // ...
                //         })
                //         .on_eos(
                //             |_trailers: Option<&HeaderMap>,
                //              _stream_duration: Duration,
                //              _span: &Span| {
                //                 // ...
                //             },
                //         )
                //         .on_failure(
                //             |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                //                 // ...
                //             },
                //         ),
                // )
                .layer(Extension(app_state)),
        )
}
