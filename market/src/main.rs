use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use market::{app_config::AppConfig, build_clients, build_routes, build_app, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Build apps config
    let config = AppConfig::build()?;

    // Initialize clients
    let clients = build_clients(&config)?;

    // Build app state
    let app: Arc<App> = build_app(config, clients).await?.into();

    // Start server
    let routes = build_routes(app);
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;

    Ok(())
}
