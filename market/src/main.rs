use std::net::SocketAddr;

use market::{build_routes, build_state, config::AppConfig};
use tower::make::Shared;

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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(app))
        .await
        .unwrap();
}
