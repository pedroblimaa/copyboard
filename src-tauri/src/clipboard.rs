use arboard::Clipboard;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const CLIPBOARD_WATCHER_DELAY_MS: u64 = 500;

pub fn start_clipboard_watcher(clipboard: Arc<Mutex<Clipboard>>) {
    thread::spawn(move || {
        let mut last_clipboard = String::new();

        loop {
            let mut clipboard = clipboard.lock().unwrap();

            if let Ok(current_text) = clipboard.get_text() {
                if current_text != last_clipboard {
                    println!("Clipboard updated: {}", current_text);

                    last_clipboard = current_text;
                }
            } else {
                println!("Failed to get clipboard text");
            }

            thread::sleep(Duration::from_millis(CLIPBOARD_WATCHER_DELAY_MS));
        }
    });
}
