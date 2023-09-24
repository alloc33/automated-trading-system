use std::str::FromStr;

use axum::{
    body::Body,
    http::{method::Method, Request},
};
use market::api::{
    alert::{AlertType, BarData, WebhookAlertData},
    price::Price,
};
use pretty_assertions::assert_eq;
use rust_decimal::Decimal;
use sqlx::PgPool;
use tower::ServiceExt;

mod setup;
use setup::make_test_app;
use uuid::Uuid;

#[sqlx::test]
pub(crate) async fn webhook(pool: PgPool) {
    let app = make_test_app(pool).await;

    let new_alert = WebhookAlertData {
        strategy_id: Uuid::from_str("559A0466-9301-4198-AB4D-0302BEAC3CC2").unwrap(),
        ticker: "AAPL".to_string(),
        timeframe: "5m".to_string(),
        exchange: "NASDAQ".to_string(),
        alert_type: AlertType::Long,
        bar: BarData {
            time: chrono::Utc::now(),
            open: Price::new(Decimal::new(17655, 2)),
            high: Price::new(Decimal::new(17658, 2)),
            low: Price::new(Decimal::new(17620, 2)),
            close: Price::new(Decimal::new(17640, 2)),
            volume: Decimal::new(113629, 3),
        },
        time: chrono::Utc::now(),
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:8000/webhook")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&new_alert).unwrap()))
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();

    assert_eq!(resp.status(), 201);
}
