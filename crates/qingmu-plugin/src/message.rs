use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum PluginMessage {
    SentToUi(SentToUiMessage),
    SentToDeno(SendToDenoMessage),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SendToDenoMessage {
    pub id: String,
    pub package: String,
    pub event: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SentToUiMessage {
    pub id: String,
    pub package: String,
    pub event: String,
    pub content: String,
}
