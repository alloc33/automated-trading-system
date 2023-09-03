use std::sync::Arc;

use tokio::sync::{
    mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EventBus {
    pub sender: Arc<UnboundedSender<Event>>,
    pub receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
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

pub enum Event {
    NewAlert(Alert),
}

#[derive(Debug)]
pub struct Alert {
    pub id: Uuid,
}
