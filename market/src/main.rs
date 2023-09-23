use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use apca::{ApiInfo, Client as AlpacaClient};
use market::{app_config::AppConfig, broker_client::Clients, build_routes, build_state, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Build apps config
    let config = AppConfig::build()?;

    // Initialize clients
    let clients = build_trade_clients(&config)?;

    // Build app state
    let state: Arc<App> = build_state(config, clients).await?.into();

    // Start server
    let app = build_routes(state);
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn build_trade_clients(config: &AppConfig) -> Result<Arc<Clients>, Box<dyn Error>> {
    let alpaca = AlpacaClient::new(ApiInfo::from_parts(
        &config.alpaca.apca_api_base_url,
        &config.alpaca.apca_api_key_id,
        &config.alpaca.apca_api_secret_key,
    )?);

    Ok(Arc::new(Clients {
        alpaca: Arc::new(alpaca),
    }))
}
