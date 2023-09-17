use std::{fmt::Debug, sync::Arc};

use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{api::alert::TradeSignal, strategy_manager::StrategyManager};

#[derive(Debug)]
pub struct EventBus {
    pub sender: UnboundedSender<Event>,
    pub receiver: Option<UnboundedReceiver<Event>>,
}

#[allow(clippy::new_without_default)]
impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<Event>();
        Self {
            sender,
            receiver: Some(receiver),
        }
    }

    pub async fn send(&self, event: Event) -> Result<(), SendError<Event>> {
        self.sender.send(event)
    }
}

/// Receive alert and pass it to the strategy manager through the event bus.
pub async fn dispatch_events(
    mut event_receiver: Option<UnboundedReceiver<Event>>,
    strategy_manager: Arc<StrategyManager>,
) {
    let mut receiver = event_receiver.take().expect("Event receiver");

    while let Some(event) = receiver.recv().await {
        match event.clone() {
            Event::WebhookAlert(alert_data) => {
                let strategy_manager = Arc::clone(&strategy_manager);
                tokio::spawn(async move {
                    if let Err(err) = strategy_manager.process_trade_signal(alert_data).await {
                        tracing::error!("Error processing trade signal: {err:?}");
                    }
                });
            } // Event::UpdateStrategy(_) => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    WebhookAlert(TradeSignal),
    // TODO: UpdateStrategy
    // UpdateStrategy(UpdateStrategy),
}
