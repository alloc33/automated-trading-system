use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use market::{
    build_routes, build_state, config::AppConfig, events::EventBus, strategy_manager,
    trade_executor::TradeExecutor,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();
    // Build apps config
    let config = AppConfig::build();

    // Build app state
    let event_bus = EventBus::new();

    // Build app state
    let state = build_state(config, Arc::clone(&event_bus.sender))
        .await
        .unwrap_or_else(|err| {
            tracing::error!(error=%err, "Cannot connect to database");
            std::process::exit(1);
        });

    // Initialize trade executor
    let trade_executor = TradeExecutor::new();

    // Run strategy manager
    tokio::spawn(strategy_manager::run(
        Arc::clone(&event_bus.receiver),
        event_bus.sender,
        trade_executor,
    ));

    // Start server
    let app = build_routes(state.into());
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
