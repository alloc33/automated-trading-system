use std::{fmt::Debug, sync::Arc};

use tokio::sync::mpsc::UnboundedReceiver;

use crate::{api::alert::TradeSignal, strategy_manager::StrategyManager};

/// Receive alert and pass it to the strategy manager through the event bus.
pub async fn dispatch_events(
    mut event_receiver: Option<UnboundedReceiver<Event>>,
    strategy_manager: Arc<StrategyManager>,
) {
    let mut receiver = event_receiver.take().expect("Event receiver");

    while let Some(event) = receiver.recv().await {
        match event.clone() {
            Event::WebhookAlert(signal) => {
                let strategy_manager = Arc::clone(&strategy_manager);
                tokio::spawn(async move {
                    if let Err(err) = strategy_manager.process_trade_signal(signal).await {
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
