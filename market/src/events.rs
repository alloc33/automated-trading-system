use std::sync::Arc;

use tokio::sync::mpsc::UnboundedReceiver;
use tracing::error;
use uuid::Uuid;

use crate::{
    api::alert::TradeSignal,
    client::Clients,
    strategy_manager::{process_market_action, process_trade_signal, Broker},
};

#[derive(Debug, Clone)]
pub enum ActionType {
    CancelOrder(Uuid),
    CancelAllOrders,
    GetAccount,
    GetPositions,
    GetOrders,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub broker: Broker,
    pub action: ActionType,
}

#[derive(Debug, Clone)]
pub enum Event {
    WebhookAlert(Box<TradeSignal>),
    ClientAction(Action),
}

pub async fn handle_events(mut event_receiver: UnboundedReceiver<Event>, clients: Arc<Clients>) {
    while let Some(event) = event_receiver.recv().await {
        let clients = Arc::clone(&clients);

        let client_for_broker = move |broker: &Broker| match broker {
            Broker::Alpaca => Arc::clone(&clients.alpaca),
        };

        match event.clone() {
            Event::WebhookAlert(signal) => {
                tokio::spawn(async move {
                    process_trade_signal(client_for_broker(&signal.strategy.broker), *signal)
                        .await
                        .unwrap_or_else(|e| error!("Error processing trade signal: {:?}", e));
                });
            }
            Event::ClientAction(action) => {
                tokio::spawn(async move {
                    process_market_action(client_for_broker(&action.broker), action.action)
                        .await
                        .unwrap_or_else(|e| error!("Error processing manual action: {:?}", e));
                });
            }
        }
    }
}
