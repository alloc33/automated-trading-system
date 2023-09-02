use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use market::events::EventBus;
use market::{build_routes, build_state, config::AppConfig};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();
    // Build apps config
    let config = AppConfig::build();
    // Build app state

    let event_bus = EventBus::new();

    let state = build_state(config, event_bus.sender).await.unwrap_or_else(|err| {
        tracing::error!(error=%err, "Cannot connect to database");
        std::process::exit(1);
    });

    let event_receiver = Arc::clone(&event_bus.receiver);

    let app = build_routes(state.into());
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
