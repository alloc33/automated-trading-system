use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use market::{
    app_config::AppConfig,
    build_routes, build_state,
    events::{dispatch_events, EventBus},
    strategy_manager::StrategyManager,
    trade_executor::TradeExecutor,
    App,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Build apps config
    let config = AppConfig::build()?;

    // Build event bus
    let events = EventBus::new();

    // Build app state
    let state: Arc<App> = build_state(config, events.sender.clone()).await?.into();

    // Setup trading related components
    let trade_executor = TradeExecutor::new(Arc::clone(&state));

    let strategy_manager = Arc::new(StrategyManager::new(trade_executor));

    // Start event dispatcher
    tokio::spawn(dispatch_events(events.receiver, strategy_manager));

    // Start server
    let app = build_routes(state);
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
