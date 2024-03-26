use serde::{Deserialize, Serialize};
use tauri::{Error, Result};

use crate::utils::clipboard_image_saver;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Rtf(String),
    Html(String),
    Image((String, String)),
    Files(Vec<String>),
    Other(String, Vec<u8>),
}

impl ClipboardContent {
    pub fn from(data: clipboard_rs::ClipboardContent) -> Result<Self> {
        match data {
            clipboard_rs::ClipboardContent::Text(data) => Ok(ClipboardContent::Text(data)),
            clipboard_rs::ClipboardContent::Rtf(data) => Ok(ClipboardContent::Rtf(data)),
            clipboard_rs::ClipboardContent::Html(data) => Ok(ClipboardContent::Html(data)),
            clipboard_rs::ClipboardContent::Image(data) => match clipboard_image_saver(data) {
                Ok(data) => {
                    let (path, thumbnail_path) = data;
                    Ok(ClipboardContent::Image((path, thumbnail_path)))
                }
                Err(e) => return Err(Error::Anyhow(anyhow::anyhow!(e.to_string()))),
            },
            clipboard_rs::ClipboardContent::Files(data) => Ok(ClipboardContent::Files(data)),
            clipboard_rs::ClipboardContent::Other(format, data) => {
                Ok(ClipboardContent::Other(format, data))
            }
        }
    }
}
