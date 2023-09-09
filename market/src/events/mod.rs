use std::{fmt::Debug, sync::Arc};

use thiserror::Error as ThisError;
use tokio::sync::{
    mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tracing::error;

use crate::{api::alert::AlertData, strategy_manager::trade_error::TradeError};

#[derive(Clone, Debug)]
pub struct EventBus {
    pub sender: UnboundedSender<Event>,
    pub receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
}

#[derive(Debug, ThisError)]
pub enum HandleEventError {
    #[error("Trade error - {0}")]
    TradeError(TradeError),
    #[error("Unknown exchange - {0}")]
    UnknownExchange(String),
}

#[axum::async_trait]
pub trait EventHandler {
    type EventPayload;

    async fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError>;
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

pub async fn dispatch_events(
    event_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
    trade_signal_handler: Arc<dyn EventHandler<EventPayload = AlertData> + Send + Sync>,
) {
    let mut receiver = event_receiver.lock().await;
    while let Some(event) = receiver.recv().await {
        match event.clone() {
            Event::WebhookAlert(signal) => {
                let trade_signal_handler = Arc::clone(&trade_signal_handler);

                tokio::spawn(async move {
                    if let Err(err) = trade_signal_handler.handle_event(&signal).await {
                        error!(error = ?err, "Cannot process trading signal");
                    }
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    WebhookAlert(AlertData), // TODO: add more events. ?manual trades
}
