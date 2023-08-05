use std::net::{Ipv4Addr, SocketAddr};
use market::{build_routes, build_state, config::AppConfig};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();
    let config = AppConfig::build();
    let state = build_state(config).await.unwrap_or_else(|err| {
        tracing::error!(error=%err, "Cannot connect to database");
        std::process::exit(1);
    });
    let app = build_routes(state.into());
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
