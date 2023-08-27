use axum::http::{self, Request};
use http::method::Method;
use hyper::{Body, StatusCode};
use market::{
    api::alert::{AlertType, BarData, NewTradingAlert},
    objects::Price,
};
use pretty_assertions::assert_eq;
use rust_decimal::Decimal;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;

mod setup;
use setup::make_test_app;

#[sqlx::test]
pub(crate) async fn create_alerts(pool: PgPool) {
    let app = make_test_app(pool).await;

    let new_alert = NewTradingAlert {
        webhook_key: "a773a6b1e4bf06be5c2983f705252eaeafc2c6e635a5a3beb2901a4cf40459a2".to_string(),
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
        .uri("http://localhost:8000/alert")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&new_alert).unwrap()))
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();

    assert_eq!(resp.status(), 201);
}

#[sqlx::test(fixtures("alerts"))]
pub(crate) async fn get_alerts(pool: PgPool) {
    let app = make_test_app(pool).await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8000/alert")
        .header(
            "Authorization",
            "hcYXTtlU67hjLdQ5LWbG8FG6qP2GDdEiBj8Oh+NMijs=",
        )
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = hyper::body::to_bytes(resp.into_body())
        .await
        .expect("Failed to read response body");

    let body_str =
        String::from_utf8(body_bytes.to_vec()).expect("Failed to convert response body to string");
    let json_response: Value =
        serde_json::from_str(&body_str).expect("Failed to parse JSON response");

    let alerts = json_response.get("results").expect("Failed to get results").as_array().unwrap();

    assert_eq!(alerts.len(), 8);
}
