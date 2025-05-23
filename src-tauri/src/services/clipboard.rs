use arboard::Clipboard;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{api::dropbox, services::auth_service, utils::file_utils};

const CLIPBOARD_WATCHER_DELAY_MS: u64 = 500;
type SharedClipboard = Arc<Mutex<Clipboard>>;
type SharedLastClip = Arc<Mutex<String>>;

pub fn start_clipboard_watcher(clipboard: SharedClipboard, last_clipboard: SharedLastClip) {
    thread::spawn(move || loop {
        handle_clipboard_iteration(&clipboard, &last_clipboard);
        thread::sleep(Duration::from_millis(CLIPBOARD_WATCHER_DELAY_MS));
    });
}

// Implemented, needs testing
pub fn upadate_clipboard(clipboard: &SharedClipboard, last_clipboard: &SharedLastClip, text: &str) {
    let mut clipboard = clipboard.lock().unwrap();
    clipboard.set_text(text.to_string()).unwrap();
    let mut last_clipboard = last_clipboard.lock().unwrap();
    *last_clipboard = text.to_string();
}

fn handle_clipboard_iteration(clipboard: &SharedClipboard, last_clipboard: &SharedLastClip) {
    let mut clipboard = clipboard.lock().unwrap();
    let mut last_clipboard = last_clipboard.lock().unwrap();
    let last_text = match clipboard.get_text().ok() {
        Some(text) => text,
        None => "".to_string(),
    };

    let current_text = match clipboard.get_text().ok() {
        Some(text) => text,
        None => return,
    };

    if last_text != current_text {
        *last_clipboard = current_text.clone();
        let text_file = file_utils::create_temp_file(&last_clipboard).unwrap();

        let token = auth_service::get_token().unwrap();
        dropbox::upload_file(token, text_file);
    }
}
