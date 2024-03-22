use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{error::SendError, Sender},
    Mutex,
};

use uuid::Uuid;

pub mod messages;

type EventGroup = HashMap<Uuid, Sender<String>>;

#[derive(Clone, Default)]
pub struct EventsManager {
    listeners: Arc<Mutex<HashMap<String, EventGroup>>>,
}

impl EventsManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn listen_on(&self, name: String, listener_id: Uuid, listener: Sender<String>) {
        if !self.listeners.lock().await.contains_key(&name) {
            self.listeners
                .lock()
                .await
                .insert(name.clone(), HashMap::new());
        }

        self.listeners
            .lock()
            .await
            .get_mut(&name)
            .unwrap()
            .insert(listener_id, listener.clone());
    }

    pub async fn unlisten_from(&self, name: String, listener_id: Uuid) {
        self.listeners
            .lock()
            .await
            .get_mut(&name)
            .unwrap()
            .remove(&listener_id);
    }
    pub async fn send(&self, name: String, content: String) -> Result<(), SendError<String>> {
        let all_listeners = self.listeners.lock().await;
        let listeners = all_listeners.get(&name);
        if let Some(listeners) = listeners {
            for listener in listeners.values() {
                listener.send(content.clone()).await?;
            }
        }
        Ok(())
    }
}
