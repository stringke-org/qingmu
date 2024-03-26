use std::path::PathBuf;

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat, RustImageData};
use clipboard_rs::common::RustImage;
use tauri::{Error, Result};

use crate::models::clipboard::ClipboardContent;
use crate::models::config::AppConfig;
use crate::state::AppState;
use crate::utils::clipboard_image_saver;

#[tauri::command]
pub fn get_app_config(state: tauri::State<'_, AppState>) -> AppConfig {
    state.0.lock().unwrap().config.clone()
}

#[tauri::command]
pub fn save_app_config(state: tauri::State<'_, AppState>, config: AppConfig) {
    state.0.lock().unwrap().config = config;
    state.0.lock().unwrap().config.save();
}

#[tauri::command]
pub fn get_clipboard_text() -> Result<String> {
    let clipboard_context = ClipboardContext::new().unwrap();
    match clipboard_context.get_text() {
        Ok(str) => {
            if str.is_empty() {
                Err(Error::from(anyhow::anyhow!("Clipboard text is empty")))
            } else {
                Ok(str)
            }
        }
        Err(_) => Err(Error::from(anyhow::anyhow!("Failed to get clipboard text"))),
    }
}

#[tauri::command]
pub fn set_clipboard_text(text: String) -> Result<()> {
    let clipboard_context = ClipboardContext::new().unwrap();
    clipboard_context.set_text(text).unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_clipboard_image() -> Result<(String, String)> {
    let clipboard_context = ClipboardContext::new().unwrap();
    match clipboard_context.get_image() {
        Ok(data) => match clipboard_image_saver(data) {
            Ok((path, thumbnail_path)) => Ok((path, thumbnail_path)),
            Err(e) => Err(Error::from(e)),
        },
        Err(_) => Err(Error::from(anyhow::anyhow!(
            "Failed to get clipboard image"
        ))),
    }
}

#[tauri::command]
pub fn set_clipboard_image(data: String) -> Result<()> {
    let clipboard_context = ClipboardContext::new().unwrap();
    let image = RustImageData::from_path(&*data);
    match image {
        Ok(image) => {
            clipboard_context.set_image(image).unwrap();
            Ok(())
        }
        Err(_) => Err(Error::from(anyhow::anyhow!(
            "Failed to set clipboard image"
        ))),
    }
}

#[tauri::command]
pub fn get_clipboard_html() -> Result<String> {
    let clipboard_context = ClipboardContext::new().unwrap();
    match clipboard_context.get_html() {
        Ok(str) => {
            if str.is_empty() {
                Err(Error::from(anyhow::anyhow!("Clipboard html is empty")))
            } else {
                Ok(str)
            }
        }
        Err(_) => Err(Error::from(anyhow::anyhow!("Failed to get clipboard html"))),
    }
}

#[tauri::command]
pub fn set_clipboard_html(html: String) -> Result<()> {
    let clipboard_context = ClipboardContext::new().unwrap();
    clipboard_context.set_html(html).unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_clipboard_rtf() -> Result<String> {
    let clipboard_context = ClipboardContext::new().unwrap();
    match clipboard_context.get_rich_text() {
        Ok(str) => {
            if str.is_empty() {
                Err(Error::from(anyhow::anyhow!("Clipboard rtf is empty")))
            } else {
                Ok(str)
            }
        }
        Err(_) => Err(Error::from(anyhow::anyhow!("Failed to get clipboard rtf"))),
    }
}

#[tauri::command]
pub fn set_clipboard_rtf(rtf: String) -> Result<()> {
    let clipboard_context = ClipboardContext::new().unwrap();
    clipboard_context.set_rich_text(rtf).unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_clipboard_files() -> Result<Vec<String>> {
    let clipboard_context = ClipboardContext::new().unwrap();
    match clipboard_context.get_files() {
        Ok(files) => Ok(files),
        Err(_) => Err(Error::from(anyhow::anyhow!(
            "Failed to get clipboard files"
        ))),
    }
}

#[tauri::command]
pub fn set_clipboard_files(files: Vec<String>) -> Result<()> {
    let clipboard_context = ClipboardContext::new().unwrap();
    clipboard_context.set_files(files).unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_clipboard_format(format: String) -> Result<Vec<ClipboardContent>> {
    let clipboard_context = ClipboardContext::new().unwrap();
    let formats = vec![ContentFormat::Other(format.to_string())];

    match clipboard_context.get(formats.as_slice()) {
        Ok(items) => {
            let mut result = Vec::new();
            for item in items {
                match ClipboardContent::from(item) {
                    Ok(content) => result.push(content),
                    Err(e) => return Err(Error::from(e)),
                }
            }
            Ok(result)
        }
        Err(_) => Err(Error::from(anyhow::anyhow!(
            "Failed to get clipboard format"
        ))),
    }
}

#[tauri::command]
pub fn set_clipboard_format(format: String, data: Vec<u8>) -> Result<()> {
    let clipboard_context = ClipboardContext::new().unwrap();
    match clipboard_context.set_buffer(&*format, data) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::from(anyhow::anyhow!(
            "Failed to set clipboard format"
        ))),
    }
}

#[tauri::command]
pub fn guess_lang(
    state: tauri::State<'_, AppState>,
    content: String,
    path: Option<String>,
) -> Option<String> {
    match path {
        Some(path) => state
            .0
            .lock()
            .unwrap()
            .guesslang_manager
            .by_path(&*PathBuf::from(path)),
        None => state
            .0
            .lock()
            .unwrap()
            .guesslang_manager
            .by_content(&content),
    }
}
