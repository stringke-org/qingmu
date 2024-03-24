use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::message::PluginMessage;

pub struct EventSender {
    sender: UnboundedSender<PluginMessage>,
}

impl Clone for EventSender {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl EventSender {
    pub fn send(&mut self, message: PluginMessage) -> Result<(), SendError<PluginMessage>> {
        self.sender.send(message)
    }
}

pub struct EventReceiver {
    receiver: UnboundedReceiver<PluginMessage>,
}

impl EventReceiver {
    pub async fn recv(&mut self) -> Option<PluginMessage> {
        self.receiver.recv().await
    }
}

pub fn create_event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    (EventSender { sender }, EventReceiver { receiver })
}
