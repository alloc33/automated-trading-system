use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use market::{
    build_routes, build_state,
    config::AppConfig,
    events::{dispatch_events, EventBus},
    strategy_manager::StrategyManager,
    trade_executor::TradeExecutor,
    App,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();
    // Build apps config
    let config = AppConfig::build();

    // Build event bus
    let event_bus = EventBus::new();

    // Build app state
    let state: Arc<App> = build_state(config, Arc::clone(&event_bus.sender))
        .await
        .unwrap_or_else(|err| {
            tracing::error!(error=%err, "Cannot connect to database");
            std::process::exit(1);
        })
        .into();

    // Setup trading related components
    let trade_executor = TradeExecutor::new(Arc::clone(&state));
    let strategy_manager = Arc::new(StrategyManager::new(trade_executor));

    // Start event dispatcher
    tokio::spawn(dispatch_events(event_bus.receiver, strategy_manager));

    // Start server
    let app = build_routes(state);
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
