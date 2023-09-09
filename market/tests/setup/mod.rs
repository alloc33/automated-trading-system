use axum::Router;
use market::{build_routes, config::AppConfig, events::EventBus, App};
use sqlx::PgPool;

pub async fn make_test_app(pool: PgPool) -> Router {
    std::env::set_var("WEBHOOK_KEY", "y2qRr/c9R3aPmeBSNRUbmp8HFevQ3bSlWi+TKLj5AhU");
    dotenvy::dotenv().ok();

    let config = AppConfig {
        database_url: "postgres://market_app:@localhost:5432/market_db".to_string(),
        api_key: "hcYXTtlU67hjLdQ5LWbG8FG6qP2GDdEiBj8Oh+NMijs=".to_string(),
        trade_signal_max_retries: 3,
        trade_signal_retry_delay: 0.5,
    };

    let event_bus = EventBus::new();

    build_routes(std::sync::Arc::new(App {
        db: pool,
        event_sender: event_bus.sender,
        config,
    }))
}
