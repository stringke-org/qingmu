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
