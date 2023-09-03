pub mod macd_ema_v0;
pub mod trade_error;

use std::sync::Arc;

use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tracing::{debug, error};

use crate::{
    events::{Event, TradingSignal},
    trade_executor::TradeExecutor,
};

use self::trade_error::TradeError;

pub async fn run(
    event_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
    event_sender: Arc<UnboundedSender<Event>>,
    trade_executor: TradeExecutor,
) {
    let mut receiver = event_receiver.lock().await;

    let trade_executor = Arc::new(trade_executor);

    while let Some(event) = receiver.recv().await {
        tokio::spawn(handle_event(
            event_sender.clone(),
            event,
            Arc::clone(&trade_executor),
        ));
    }
}

async fn handle_event(
    sender: Arc<UnboundedSender<Event>>,
    event: Event,
    trade_executor: Arc<TradeExecutor>,
) {
    match &event {
        Event::WebhookAlert(signal) => {
            debug!(signal = ?signal, "Received WebhookAlert event");
            if let Err(err) = process_trading_signal(signal, trade_executor).await {
                error!(error = ?err, "Cannot process trading signal");
                _ = sender.send(Event::WebhookAlert(TradingSignal::StopLoss))
            }
        }
    }
}

async fn process_trading_signal(
    signal: &TradingSignal,
    trade_executor: Arc<TradeExecutor>,
) -> Result<(), TradeError> {
    Ok(())
}
