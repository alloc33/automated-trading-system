use std::net::{Ipv4Addr, SocketAddr};

use market::{build_routes, build_state, config::AppConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "market=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::build();
    let state = build_state(config).await.unwrap_or_else(|err| {
        tracing::error!(error=%err, "Cannot connect to database");
        std::process::exit(1);
    });
    let app = build_routes(state.into());
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
