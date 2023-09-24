use axum::Router;
use market::{app_config::AppConfig, build_broker_clients, build_routes, App};
use sqlx::PgPool;

pub async fn make_test_app(pool: PgPool) -> Router {
    let config = AppConfig::build_for_test().unwrap();

    let clients = build_broker_clients(&config).unwrap();

    build_routes(std::sync::Arc::new(App {
        db: pool,
        clients,
        config,
    }))
}
