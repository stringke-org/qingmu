use std::sync::Mutex;

use clipboard_rs::ClipboardContext;

use crate::manager::guesslang_manager::GuessLangManager;
use crate::models::config::AppConfig;

pub struct AppState(pub Mutex<InnerAppState>);

impl Default for AppState {
    fn default() -> Self {
        Self(Mutex::new(InnerAppState::init()))
    }
}

pub struct InnerAppState {
    pub config: AppConfig,
    pub guesslang_manager: GuessLangManager,
    pub clipboard_context: ClipboardContext,
}

impl InnerAppState {
    pub fn init() -> Self {
        Self {
            config: AppConfig::load(),
            guesslang_manager: GuessLangManager::new(),
            clipboard_context: ClipboardContext::new().unwrap(),
        }
    }
}
