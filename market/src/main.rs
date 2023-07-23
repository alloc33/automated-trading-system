use std::sync::Arc;

pub mod config;
use axum::{response::IntoResponse, routing::get, Json, Router};
use config::Config;
use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::build();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState {
            db: pool.clone(),
            env: config.clone(),
        }));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
