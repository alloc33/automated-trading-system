pub mod broker;
pub mod trade_error;

use std::sync::Arc;

use apca::{ApiInfo, Client as AlpacaClient};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use thiserror::Error as ThisError;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use trade_error::TradeError;
use uuid::Uuid;
use uuid7::uuid7;

use self::broker::Broker;
use crate::{
    api::alert::AlertData,
    events::{EventHandler, HandleEventError, Event},
    trade_executor::TradeExecutor,
    App,
};

#[derive(Debug, ThisError)]
pub enum StrategyManagerError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    AlpacaClientError(#[from] apca::Error),
    #[error("Unknown strategy - {0}")]
    UnknownStrategy(String),
    #[error("Unknown exchange - {0}")]
    UnknownExchange(String),
    #[error("Strategy {0} with id {1} is disabled")]
    StrategyDisabled(String, String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Strategy {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub broker: Broker,
    pub max_order_retries: u8,
    pub order_retry_delay: f64,
}

pub struct StrategyManager {
    app_state: Arc<App>,
    strategies: Vec<Strategy>,
    alpaca_client: AlpacaClient,
    trade_executor: TradeExecutor,
}

#[derive(Debug)]
pub struct Order {
    id: Uuid,
    ticker: String,
}

impl StrategyManager {
    pub fn new(
        app_state: Arc<App>,
        trade_executor: TradeExecutor,
    ) -> Result<Self, StrategyManagerError> {
        let strategies = Config::builder()
            .add_source(File::with_name("market/strategies.toml"))
            .build()
            .map_err(StrategyManagerError::ConfigError)?
            .try_deserialize::<Vec<Strategy>>()?;

        let alpaca_client = AlpacaClient::new(ApiInfo::from_parts(
            &app_state.config.alpaca.apca_api_base_url,
            &app_state.config.alpaca.apca_api_key_id,
            &app_state.config.alpaca.apca_api_secret_key,
        )?);

        Ok(Self {
            app_state,
            strategies,
            alpaca_client,
            trade_executor,
        })
    }

    fn create_order(&self, alert_data: &AlertData) -> Result<Order, StrategyManagerError> {
        // Validate strategy - check if strategy exists and it's enabled.
        let validated_strategy = self.strategies
            .iter()
            .find(|strategy| strategy.id == alert_data.strategy_id)
            .ok_or_else(|| StrategyManagerError::UnknownStrategy(alert_data.strategy_id.to_string()))?;

        if !validated_strategy.enabled {
            return Err(StrategyManagerError::StrategyDisabled(
            validated_strategy.name.clone(),
            validated_strategy.id.to_string(),
            ));
        }

        // TODO: Complete Order creation
        let order = Order {
            id: uuid7::new_v7(),
            ticker: alert_data.ticker.clone()
        };

        Ok(order)
    }
}

#[axum::async_trait]
impl EventHandler for StrategyManager {
    type EventPayload = AlertData;

    // NOTE: Have to create a method which would validate strategy and create order, because
    // strategy is a part of AlertData
    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
        let strategy = self.validate_strategy(event.strategy_id)?;

        if !strategy.enabled {
            tracing::info!("Strategy {} is disabled, ignoring event", strategy.name);
            return Ok(());
        }

        // if let Some(strategy) = self.find_strategy(event.strategy_id) {
        // let mut retries = 0;

        // let order = strategy.broker.create_order(event);

        // loop {
        //     let trade_result = self.trade_executor.execute_order(&order).await;

        //     if trade_result.is_ok() {
        //         info!("Order successfully executed");
        //         return Ok(());
        //     }

        //     retries += 1;

        //     if retries >= strategy.max_event_retries {
        //         error!("Max event retries reached, giving up.");
        //         return Err(TradeError::MaxRetriesReached(order).into());
        //     }

        //     tokio::time::sleep(Duration::from_secs_f64(strategy.event_retry_delay)).await;
        // }

        // _ = self.app_state.event_sender.send(Event::);

        Ok(())
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {} }}", self.id)
    }
}
