pub mod trade_signal_handler;

use std::{fmt::Debug, sync::Arc};

use tokio::sync::{
    mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};

use tracing::error;
use crate::strategy_manager::trade_error::TradeError;

#[derive(Clone, Debug)]
pub struct EventBus {
    pub sender: Arc<UnboundedSender<Event>>,
    pub receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
}

#[derive(Debug)]
pub enum HandleEventError {
    TradeError(TradeError),
}

pub trait EventHandler {
    type EventPayload;

    fn handle_event(&self, event: &Self::EventPayload) -> Result<(), HandleEventError>;
}

#[allow(clippy::new_without_default)]
impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<Event>();
        let sender = Arc::new(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        Self { sender, receiver }
    }

    pub async fn send(&self, event: Event) -> Result<(), SendError<Event>> {
        self.sender.send(event)
    }
}

pub async fn dispatch_events(
    event_bus: Arc<EventBus>,
    trade_signal_handler: Arc<dyn EventHandler<EventPayload = TradingSignal>>,
) {
    let mut receiver = event_bus.receiver.lock().await;
    while let Some(event) = receiver.recv().await {
        match &event {
            Event::WebhookAlert(signal) => {
                if let Err(err) = trade_signal_handler.handle_event(signal) {
                    error!(error = ?err, "Cannot process trading signal");
                    _ = event_bus.sender.send(event)
                }
            }
        }
    }
}

pub enum Event {
    WebhookAlert(TradingSignal), // TODO: add more events. ?manual trades
}

#[derive(Debug)]
pub enum TradingSignal {
    Long,
    Short,
    StopLoss,
    TakeProfit,
}
