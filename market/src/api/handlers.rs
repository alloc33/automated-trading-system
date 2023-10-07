use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;

use super::{
    error::ApiError,
    objects::{
        Account, ActivitiesRequest, Activity, Asset, AssetClass, Broker, Order, OrdersRequest,
        Position,
    },
    Response,
};
use crate::{alert::WebhookAlertData, clients::BrokerClient, trade_signal::TradeSignal, App};

pub async fn receive_webhook_alert(
    State(app): State<Arc<App>>,
    WithRejection(alert_data, _): WithRejection<Json<WebhookAlertData>, ApiError>,
) -> Response<()> {
    let strategy_manager = Arc::clone(&app.strategy_manager);
    let trade_signal = TradeSignal::from_alert_data(alert_data.0.clone(), &app.config)?;

    tokio::spawn(async move {
        if let Err(err) = strategy_manager.process_trade_signal(trade_signal).await {
            error!("Failed to process trade signal, error: {:?}", err);
        };
    });

    // app.strategy_manager.

    // let client = match &trade_signal.strategy.broker {
    //     Broker::Alpaca => Arc::clone(&app.clients.alpaca),
    // };

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

    Ok(Json::default())
}

#[derive(Debug, Deserialize)]
pub struct BrokerQuery {
    broker: Broker,
}

#[derive(Debug, Deserialize)]
pub struct AssetTypeQuery {
    class: AssetClass,
}

pub async fn check_health() -> Response<()> {
    Ok(Json::default())
}

pub async fn get_account(
    State(app): State<Arc<App>>,
    Query(broker_query): Query<BrokerQuery>,
) -> Response<Account> {
    let client = broker_query.broker.get_client(&app);
    Ok(Json(client.get_account().await?))
}

#[axum::debug_handler]
pub async fn get_activities(
    State(app): State<Arc<App>>,
    WithRejection(activities_req, _): WithRejection<Json<ActivitiesRequest>, ApiError>,
) -> Response<Vec<Activity>> {
    let activities: Vec<Activity> = match activities_req.0 {
        ActivitiesRequest::AlpacaActivitiesReq(req) => {
            app.clients.alpaca.get_activities(req).await?
        }
    };

    Ok(Json(activities))
}

pub async fn get_asset(
    State(app): State<Arc<App>>,
    Query(broker_query): Query<BrokerQuery>,
    Path(symbol): Path<String>,
) -> Response<Asset> {
    let client = broker_query.broker.get_client(&app);
    Ok(Json(client.get_asset(symbol.to_uppercase()).await?))
}

pub async fn get_assets(
    State(app): State<Arc<App>>,
    Query(broker_query): Query<BrokerQuery>,
    Query(asset_type): Query<AssetTypeQuery>,
) -> Response<Vec<Asset>> {
    let client = broker_query.broker.get_client(&app);
    Ok(Json(client.get_assets(asset_type.class).await?))
}

pub async fn get_order(
    State(app): State<Arc<App>>,
    Query(broker_query): Query<BrokerQuery>,
    Path(id): Path<Uuid>,
) -> Response<Order> {
    let client = broker_query.broker.get_client(&app);
    let order = client.get_order_by_client_id(id.to_string()).await?;
    Ok(Json(order))
}

pub async fn get_orders(
    State(app): State<Arc<App>>,
    WithRejection(orders_req, _): WithRejection<Json<OrdersRequest>, ApiError>,
) -> Response<Vec<Order>> {
    let orders: Vec<Order> = match orders_req.0 {
        OrdersRequest::AlpacaOrders(req) => app.clients.alpaca.get_orders(req).await?,
    };
    Ok(Json(orders))
}

// NOTE: Algorithmically create orders
// pub async fn create_order(
//     State(app): State<Arc<App>>,
//     WithRejection(new_order_req, _): WithRejection<Json<NewOrder>, ApiError>,
// ) -> Response<Order> { let new_order = match new_order_req.0 { NewOrder::AlpacaNewOrder(req) =>
//   app.clients.alpaca.create_order(req).await?, }; Ok(Json(new_order))
// }

// NOTE: Algorithmically update orders
// pub async fn update_order(
//     Path(id): Path<Uuid>,
//     State(app): State<Arc<App>>,
//     WithRejection(order_update_req, _): WithRejection<Json<UpdateOrder>, ApiError>,
// ) -> Response<Order> { let updated_order = match order_update_req.0 {
//   UpdateOrder::AlpacaUpdateOrder(req) => app.clients.alpaca.update_order(id, req).await?, };
//   Ok(Json(updated_order))
// }

// NOTE: Algorithmically delete orders
// pub async fn delete_order(
//     State(app): State<Arc<App>>,
//     Query(broker_query): Query<BrokerQuery>,
//     Path(id): Path<Uuid>,
// ) -> Response<()> { let client = broker_query.broker.get_client(&app);
//   client.delete_order(id).await?; Ok(Json::default())
// }

pub async fn get_position(
    State(app): State<Arc<App>>,
    Path(symbol): Path<String>,
    Query(query): Query<BrokerQuery>,
) -> Response<Position> {
    let client = query.broker.get_client(&app);
    let position = client.get_position(symbol).await?;
    Ok(Json(position))
}

pub async fn get_positions(
    State(app): State<Arc<App>>,
    Query(query): Query<BrokerQuery>,
) -> Response<Vec<Position>> {
    let client = query.broker.get_client(&app);
    let positions = client.get_positions().await?;
    Ok(Json(positions))
}

pub async fn delete_position(
    State(app): State<Arc<App>>,
    Query(broker_query): Query<BrokerQuery>,
    Path(symbol): Path<String>,
) -> Response<Order> {
    let client = broker_query.broker.get_client(&app);
    let delete_position_order = client.delete_position(symbol).await?;
    Ok(Json(delete_position_order))
}
