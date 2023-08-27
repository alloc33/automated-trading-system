use axum::Router;
use hyper::{
    client::{Client, HttpConnector},
    Body,
};
use market::{build_routes, config::AppConfig, App};
use sqlx::PgPool;
use tokio::net::TcpListener;

pub async fn make_test_app(pool: PgPool) -> Router {
    std::env::set_var("WEBHOOK_KEY", "y2qRr/c9R3aPmeBSNRUbmp8HFevQ3bSlWi+TKLj5AhU");
    dotenvy::dotenv().ok();

    let config = AppConfig::default();
    build_routes(std::sync::Arc::new(App { db: pool, config }))
}
