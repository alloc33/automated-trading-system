use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::WithRejection;
use uuid::Uuid;

use super::{error::ApiError, Response};
use crate::{alert::AlertData, app_config::Strategy, events::Event, App};

pub async fn receive_alert(
    State(app): State<Arc<App>>,
    WithRejection(alert_data, _): WithRejection<Json<AlertData>, ApiError>,
) -> Response<()> {
    validate_strategy(alert_data.strategy_id, &app.config.strategies)?;

    _ = app
        .event_sender
        .send(Event::WebhookAlert(alert_data.0.clone()));

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

fn validate_strategy(strategy_id: Uuid, strategies: &[Strategy]) -> Result<(), ApiError> {
    let validated_strategy = strategies
        .iter()
        .find(|strategy| strategy.id == strategy_id)
        .ok_or_else(|| {
            let msg = format!("Unknown strategy - {}", strategy_id);
            tracing::error!(msg);
            ApiError::BadRequest(msg)
        })?;

    if !validated_strategy.enabled {
        let msg = format!(
            "Strategy {} with id {} is disabled",
            validated_strategy.name, validated_strategy.id
        );
        tracing::error!(msg);
        return Err(ApiError::BadRequest(msg));
    }

    Ok(())
}
