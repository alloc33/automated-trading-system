use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use tracing::error;

use super::{alert::TradeSignal, error::ApiError, objects::Account, Response};
use crate::{
    alert::WebhookAlertData,
    broker_client::BrokerClient,
    strategy_manager::{process_trade_signal, Broker},
    App,
};

pub async fn receive_webhook_alert(
    State(app): State<Arc<App>>,
    WithRejection(alert_data, _): WithRejection<Json<WebhookAlertData>, ApiError>,
) -> Response<()> {
    let trade_signal = TradeSignal::from_alert_data(alert_data.0.clone(), &app.config)?;

    let client = match &trade_signal.strategy.broker {
        Broker::Alpaca => Arc::clone(&app.clients.alpaca),
    };

    tokio::spawn(async {
        if let Err(err) = process_trade_signal(client, trade_signal).await {
            error!("Failed to process trade signal, error: {:?}", err);
        };
    });

    _ = sqlx::query!(
        r#"
        INSERT INTO alerts (
            alert_id,
            ticker,
            timeframe,
            exchange, 
            alert_type,
            bar_time,
            bar_open,
            bar_high,
            bar_low,
            bar_close,
            bar_volume,
            alert_fire_time,
            created_at,
            modified_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW()
        )
        "#,
        uuid7::new_v7(),
        alert_data.ticker,
        alert_data.timeframe,
        alert_data.exchange,
        alert_data.alert_type.as_ref(),
        alert_data.bar.time,
        alert_data.bar.open.as_ref(),
        alert_data.bar.high.as_ref(),
        alert_data.bar.low.as_ref(),
        alert_data.bar.close.as_ref(),
        alert_data.bar.volume,
        alert_data.time
    )
    .execute(&app.db)
    .await?;

    Ok((StatusCode::OK, Json::default()))
}

#[derive(Debug, Deserialize)]
pub struct BrokerQuery {
    broker: Broker,
}

pub async fn get_account(
    State(app): State<Arc<App>>,
    Query(query): Query<BrokerQuery>,
) -> Response<Account> {
    let client = match query.broker {
        Broker::Alpaca => &app.clients.alpaca,
    };

    let account = client.get_account().await?;

    Ok((StatusCode::OK, Json(account)))
}

pub async fn get_assets(
    State(app): State<Arc<App>>,
    Query(query): Query<BrokerQuery>,
) -> Response<()> {
    Ok((StatusCode::OK, Json::default()))
}

pub async fn get_orders(
    State(app): State<Arc<App>>,
    Query(query): Query<BrokerQuery>,
) -> Response<()> {
    Ok((StatusCode::OK, Json::default()))
}

pub async fn get_positions(
    State(app): State<Arc<App>>,
    Query(query): Query<BrokerQuery>,
) -> Response<()> {
    Ok((StatusCode::OK, Json::default()))
}
