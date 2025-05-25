use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    api::dropbox,
    models::clipboard::{SharedClipboard, SharedLastClip, StartClipboardWatcherInfo},
    services::auth_service,
    utils::file_utils,
};

const CLIPBOARD_WATCHER_DELAY_MS: u64 = 500;

pub fn start_clipboard_watcher(info: &StartClipboardWatcherInfo) {
    let clipboard = info.clipboard.clone();
    let last_clipboard = info.last_clipboard.clone();
    let file_pushed = info.file_pushed.clone();

    thread::spawn(move || loop {
        handle_clipboard_iteration(
            clipboard.clone(),
            last_clipboard.clone(),
            file_pushed.clone(),
        );
        thread::sleep(Duration::from_millis(CLIPBOARD_WATCHER_DELAY_MS));
    });
}

pub fn upadate_clipboard(clipboard: SharedClipboard, last_clipboard: SharedLastClip, text: &str) {
    let mut clipboard = clipboard.lock().unwrap();
    clipboard.set_text(text.to_string()).unwrap();

    let mut last_clipboard = last_clipboard.lock().unwrap();
    *last_clipboard = text.to_string();
}

fn handle_clipboard_iteration(
    clipboard: SharedClipboard,
    last_clipboard: SharedLastClip,
    file_pushed: Arc<Mutex<bool>>,
) {
    let mut clipboard = clipboard.lock().unwrap();
    let mut last_clipboard = last_clipboard.lock().unwrap();
    let last_text = last_clipboard.clone();

    if last_text.is_empty() {
        *last_clipboard = clipboard.get_text().unwrap_or_default();
        return;
    }

    let current_text = match clipboard.get_text().ok() {
        Some(text) => text,
        None => return,
    };

    if last_text != current_text {
        *last_clipboard = current_text.clone();
        let text_file = file_utils::create_temp_file(&last_clipboard).unwrap();

        let token = auth_service::get_token().unwrap();

        println!("Uploading file...");

        let mut file_pushed = file_pushed.lock().unwrap();
        *file_pushed = true;

        dropbox::upload_file(token, text_file);
    }
}
