use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use clipboard_rs::{ClipboardWatcher, ClipboardWatcherContext};

use crate::manager::clipboard_manager::{ClipboardManager, ClipboardManagerProxy};
use crate::models::config::AppConfig;

pub static APP_CONFIG: OnceLock<Arc<Mutex<AppConfig>>> = OnceLock::new();
pub static CLIPBOARD_MANAGER: OnceLock<Arc<Mutex<ClipboardManager>>> = OnceLock::new();
pub static CLIPBOARD_WATCHER: OnceLock<Arc<Mutex<ClipboardWatcherContext<ClipboardManagerProxy>>>> =
    OnceLock::new();

pub async fn init_clipboard() {
    let manager = Arc::new(Mutex::new(ClipboardManager::new()));
    let proxy = ClipboardManagerProxy {
        manager: manager.clone(),
    };

    let mut watcher = ClipboardWatcherContext::new().unwrap();
    watcher.add_handler(proxy);

    CLIPBOARD_WATCHER
        .set(Arc::new(Mutex::new(watcher)))
        .unwrap_or_else(|_| panic!("Failed to initialize CLIPBOARD_WATCHER"));

    tauri::async_runtime::spawn(async move {
        CLIPBOARD_WATCHER
            .get()
            .expect("Failed to get CLIPBOARD_WATCHER")
            .lock()
            .expect("Failed to lock CLIPBOARD_WATCHER")
            .start_watch();
    });

    CLIPBOARD_MANAGER
        .set(manager)
        .unwrap_or_else(|_| panic!("Failed to initialize CLIPBOARD_MANAGER"));
}

pub async fn init_global(_app: &mut tauri::App) {
    init_clipboard().await;

    APP_CONFIG
        .set(Arc::new(Mutex::new(AppConfig::load())))
        .unwrap_or_else(|_| panic!("Failed to initialize APP_CONFIG"));
}

pub fn get_config() -> MutexGuard<'static, AppConfig> {
    APP_CONFIG
        .get()
        .expect("Failed to get APP_CONFIG")
        .lock()
        .expect("Failed to lock APP_CONFIG")
}

pub fn get_clipboard_manager() -> MutexGuard<'static, ClipboardManager> {
    CLIPBOARD_MANAGER
        .get()
        .expect("Failed to get CLIPBOARD_MANAGER")
        .lock()
        .expect("Failed to lock CLIPBOARD_MANAGER")
}
