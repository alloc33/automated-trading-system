pub mod trade_error;

use std::sync::Arc;

use tokio::time::{sleep, Duration};
use tracing::{error, info};
use trade_error::TradeError;
use uuid::Uuid;

use crate::{
    api::alert::AlertData,
    app_config::Strategy,
    events::{EventHandler, HandleEventError},
    trade_executor::TradeExecutor,
    App,
};

pub struct StrategyManager {
    app_state: Arc<App>,
    trade_executor: TradeExecutor,
}

#[derive(Debug)]
pub struct Order {
    id: Uuid,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {} }}", self.id)
    }
}

impl StrategyManager {
    pub fn new(app_state: Arc<App>, trade_executor: TradeExecutor) -> Self {
        Self {
            app_state,
            trade_executor,
        }
    }

    fn find_strategy(&self, strategy_id: Uuid) -> Option<&Strategy> {
        self.app_state
            .config
            .strategies
            .iter()
            .find(|strategy| strategy.id == strategy_id)
    }

    fn create_order(&self, input: &AlertData) -> Order {
        todo!()
    }
}

#[axum::async_trait]
impl EventHandler for StrategyManager {
    type EventPayload = AlertData;

    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError> {
        if let Some(strategy) = self.find_strategy(event.strategy_id) {
            let order = self.create_order(event);
            let mut retries = 0;

            loop {
                let trade_result = self.trade_executor.execute_trade(&order).await;

                if trade_result.is_ok() {
                    info!("Trade successfully executed");
                    return Ok(());
                }

                retries += 1;

                if retries >= strategy.max_event_retries {
                    error!("Max event retries reached, giving up.");
                    return Err(TradeError::MaxRetriesReached(order).into());
                }

                tokio::time::sleep(Duration::from_secs_f64(strategy.event_retry_delay)).await;
            }
        } else {
            Err(HandleEventError::UnknownStrategy(
                event.strategy_id.to_string(),
            ))
        }
    }
}
