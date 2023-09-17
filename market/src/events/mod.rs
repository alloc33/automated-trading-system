use std::{fmt::Debug, sync::Arc};

use thiserror::Error as ThisError;
use tokio::sync::{
    mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tracing::error;

use crate::{
    api::{alert::AlertData, strategy::UpdateStrategy},
    strategy_manager::{trade_error::TradeError, StrategyManager, StrategyManagerError},
};

#[derive(Clone, Debug)]
pub struct EventBus {
    pub sender: UnboundedSender<Event>,
    pub receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
}

#[allow(clippy::new_without_default)]
impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<Event>();
        let receiver = Arc::new(Mutex::new(receiver));

        Self { sender, receiver }
    }

    pub async fn send(&self, event: Event) -> Result<(), SendError<Event>> {
        self.sender.send(event)
    }
}

/// Receive alert and pass it to the strategy manager through the event bus.
pub async fn dispatch_events(
    event_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
    strategy_manager: Arc<StrategyManager>,
) {
    let mut receiver = event_receiver.lock().await;
    while let Some(event) = receiver.recv().await {
        match event.clone() {
            Event::WebhookAlert(alert_data) => {
                let strategy_manager = Arc::clone(&strategy_manager);
                tokio::spawn(async move {
                    strategy_manager.process_trade_signal(alert_data).await;
                });
            }
            Event::UpdateStrategy(_) => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    WebhookAlert(AlertData),
    // TODO: UpdateStrategy
    UpdateStrategy(UpdateStrategy),
}
