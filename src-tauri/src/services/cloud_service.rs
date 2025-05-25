use std::{thread, time::Duration};

use crate::{
    api::dropbox,
    models::clipboard::{SharedClipboard, SharedLastClip, StartClipboardWatcherInfo},
    services::clipboard_service,
};

use super::auth_service;

pub fn start_dropbox_file_watch(info: StartClipboardWatcherInfo) {
    let file_pushed = info.file_pushed.clone();
    let clipboard = info.clipboard.clone();
    let last_clipboard = info.last_clipboard.clone();

    thread::spawn(move || {
        let token = auth_service::get_token().unwrap_or_default();

        if token.is_empty() {
            thread::sleep(Duration::from_secs(5));
            start_dropbox_file_watch(info);
            return;
        }

        let get_file_response = dropbox::get_clipboard_file(token).unwrap();
        let mut cursor = get_file_response.cursor;

        loop {
            let result = dropbox::longpoll(&cursor).unwrap();

            let mut file_pushed = file_pushed.lock().unwrap();
            if *file_pushed {
                wait_for_backoff(result.backoff);
                *file_pushed = false;
                update_cursor(None, &mut cursor);
                continue;
            }

            if result.changes {
                handle_file_change(&mut cursor, clipboard.clone(), last_clipboard.clone());
            }

            wait_for_backoff(result.backoff);
        }
    });
}

fn handle_file_change(
    cursor: &mut String,
    clipboard: SharedClipboard,
    last_clipboard: SharedLastClip,
) {
    let token = auth_service::get_token().unwrap();
    let file_content = dropbox::download_file(&token).unwrap();

    update_cursor(Some(&token), cursor);

    clipboard_service::upadate_clipboard(clipboard, last_clipboard, &file_content);
}

fn wait_for_backoff(backoff: Option<u32>) {
    if let Some(backoff) = backoff {
        thread::sleep(Duration::from_secs(backoff as u64));
    }
}

fn update_cursor(token: Option<&String>, cursor: &mut String) {
    let token = match token {
        Some(token) => token,
        None => &auth_service::get_token().unwrap(),
    };

    let file_response = dropbox::cursor_continue(&token, cursor).unwrap();
    *cursor = file_response.cursor;
}
