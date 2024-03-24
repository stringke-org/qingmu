use std::sync::{Arc, Mutex};

use clipboard_rs::{ClipboardContext, ClipboardHandler};

pub struct ClipboardManager {
    ctx: ClipboardContext,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            ctx: ClipboardContext::new().unwrap(),
        }
    }
}

impl ClipboardHandler for ClipboardManager {
    fn on_clipboard_change(&mut self) {
        println!("Clipboard changed");
    }
}

pub struct ClipboardManagerProxy {
    pub manager: Arc<Mutex<ClipboardManager>>,
}

impl ClipboardHandler for ClipboardManagerProxy {
    fn on_clipboard_change(&mut self) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.on_clipboard_change();
        }
    }
}
