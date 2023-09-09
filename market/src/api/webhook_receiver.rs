use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::WithRejection;

use super::{error::ApiError, Response};
use crate::{alert::AlertData, events::Event, App};

pub async fn receive_alert(
    State(app): State<Arc<App>>,
    WithRejection(alert, _): WithRejection<Json<AlertData>, ApiError>,
) -> Response<()> {
    _ = app.event_sender.send(Event::WebhookAlert(alert.0.clone()));

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
        alert.ticker,
        alert.timeframe,
        alert.exchange,
        alert.alert_type.as_ref(),
        alert.bar.time,
        alert.bar.open.as_ref(),
        alert.bar.high.as_ref(),
        alert.bar.low.as_ref(),
        alert.bar.close.as_ref(),
        alert.bar.volume,
        alert.time
    )
    .execute(&app.db)
    .await?;

    Ok((StatusCode::OK, Json::default()))
}
