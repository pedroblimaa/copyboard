use arboard::Clipboard;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const CLIPBOARD_WATCHER_DELAY_MS: u64 = 500;
type SharedClipboard = Arc<Mutex<Clipboard>>;

pub fn start_clipboard_watcher(clipboard: SharedClipboard) {
    thread::spawn(move || {
        let mut last_clipboard = String::new();

        loop {
            last_clipboard = handle_clipboard_iteration(&clipboard, last_clipboard);
            thread::sleep(Duration::from_millis(CLIPBOARD_WATCHER_DELAY_MS));
        }
    });
}

fn handle_clipboard_iteration(clipboard: &SharedClipboard, mut last_clipboard: String) -> String {
    let mut clipboard = clipboard.lock().unwrap();
    let current_text = clipboard.get_text().unwrap();

    if last_clipboard != current_text {
        println!("Clipboard changed: {}", current_text);
        last_clipboard = current_text;
    }

    last_clipboard
}
