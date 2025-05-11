use arboard::Clipboard;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct ClipboardState {
    clipboard: Arc<Mutex<Clipboard>>,
    last_clipboard: Arc<Mutex<String>>,
}

pub fn start_clipboard_watcher() {
    let clipboard = Arc::new(Mutex::new(Clipboard::new().unwrap()));
    let last_clipboard = Arc::new(Mutex::new(String::new()));

    let state = ClipboardState {
        clipboard: Arc::clone(&clipboard),
        last_clipboard: Arc::clone(&last_clipboard),
    };

    thread::spawn(move || loop {
        if let Some(text) = get_clipboard_text_if_diff(&state) {
            println!("Clipboard updated: {}", text);
        }

        thread::sleep(Duration::from_millis(500));
    });
}

fn get_clipboard_text_if_diff(state: &ClipboardState) -> Option<String> {
    let current_text = state.clipboard.lock().ok()?.get_text().ok()?;
    let mut last = state.last_clipboard.lock().ok()?;

    if *last != current_text {
        *last = current_text.clone();
        return Some(current_text);
    }

    None
}
