use chrono::{DateTime, Utc};

use crate::{
    api::{
        alert::{AlertType, BarData, WebhookAlertData},
        error::ApiError,
    },
    app_config::AppConfig,
    clients::BrokerClient,
    strategy::Strategy,
};

#[derive(Debug, Clone)]
pub struct TradeSignal {
    pub strategy: Strategy,
    pub ticker: String,
    pub timeframe: String,
    pub exchange: String,
    pub alert_type: AlertType,
    pub bar: BarData,
    pub time: DateTime<Utc>,
}

impl TradeSignal {
    pub async fn process(&self, client: impl BrokerClient) -> Result<(), ()> {
        let broker = &self.strategy.broker;
        let order_request = broker.create_order_request().await;

        // let order = client.create_order(new_order_req)

        Ok(())

    }

    pub fn from_alert_data(
        alert_data: WebhookAlertData,
        config: &AppConfig,
    ) -> Result<Self, ApiError> {
        let strategy_id = alert_data.strategy_id;

        let validated_strategy = config
            .strategies
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

        Ok(Self {
            strategy: validated_strategy.clone(),
            ticker: alert_data.ticker,
            timeframe: alert_data.timeframe,
            exchange: alert_data.exchange,
            alert_type: alert_data.alert_type,
            bar: alert_data.bar,
            time: alert_data.time,
        })
    }
}
