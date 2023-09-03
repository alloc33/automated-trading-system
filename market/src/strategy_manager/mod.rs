pub mod macd_ema_v0;

use std::sync::Arc;

use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tracing::debug;

use crate::{events::Event, trade_executor::TradeExecutor};

pub async fn run(
    event_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
    event_sender: Arc<Mutex<UnboundedSender<Event>>>,
    trade_executor: TradeExecutor,
) {
    let mut receiver = event_receiver.lock().await;

    while let Some(event) = receiver.recv().await {
        tokio::spawn(handle_event(event_sender.clone(), event));
    }
}

async fn handle_event(sender: Arc<Mutex<UnboundedSender<Event>>>, event: Event) {
    match &event {
        Event::NewAlert(alert) => {
            debug!(alert = ?alert, "Received NewAlert event");
            // TODO: process alert and retry if failed
        }
    }
}
