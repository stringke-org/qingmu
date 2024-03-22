use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum AppMessage {
    SentToDeno(String, String),
}