use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use apca::{ApiInfo, Client as AlpacaClient};
use market::{
    app_config::AppConfig,
    build_routes, build_state,
    events::{dispatch_events, Event},
    strategy_manager::StrategyManager,
    trade_executor::TradeExecutor,
    App,
};
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Build apps config
    let config = AppConfig::build()?;

    // Build events
    let (events_sender, events_receiver) = unbounded_channel::<Event>();

    // Build app state
    let state: Arc<App> = build_state(config, events_sender.clone()).await?.into();

    // Initialize clients
    let alpaca_client = initialize_clients(&state.config)?;

    // Setup trading related components
    let trade_executor = TradeExecutor { alpaca_client };
    let strategy_manager = Arc::new(StrategyManager::new(Arc::clone(&state), trade_executor)?);

    // Start event dispatcher
    tokio::spawn(dispatch_events(Some(events_receiver), strategy_manager));

    // Start server
    let app = build_routes(state);
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// TODO: Add more clients?
fn initialize_clients(config: &AppConfig) -> Result<AlpacaClient, Box<dyn Error>> {
    let alpaca_client = AlpacaClient::new(ApiInfo::from_parts(
        &config.alpaca.apca_api_base_url,
        &config.alpaca.apca_api_key_id,
        &config.alpaca.apca_api_secret_key,
    )?);

    Ok(alpaca_client)
}
