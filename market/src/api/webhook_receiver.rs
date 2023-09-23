use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::WithRejection;

use super::{alert::TradeSignal, error::ApiError, Response};
use crate::{alert::WebhookAlertData, events::Event, App};

pub async fn receive_alert(
    State(app): State<Arc<App>>,
    WithRejection(alert_data, _): WithRejection<Json<WebhookAlertData>, ApiError>,
) -> Response<()> {
    let trade_signal = TradeSignal::from_alert_data(alert_data.0.clone(), &app.config)?;

    _ = app.event_sender.send(Event::WebhookAlert(Box::new(trade_signal)));

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
