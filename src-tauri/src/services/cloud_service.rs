use crate::api::dropbox;

use super::auth_service;

pub fn start_dropbox_file_watch() {
    let token = auth_service::get_token().unwrap();

    let get_file_response = dropbox::get_clipboard_file(token).unwrap();
    let mut cursor = get_file_response.cursor;

    loop {
        let result = dropbox::longpoll(&cursor).unwrap();

        if result.changes {
            let token = auth_service::get_token().unwrap();
            let file_response = dropbox::cursor_continue(&token, cursor).unwrap();
            cursor = file_response.cursor;
            let file_content = dropbox::download_file(&token).unwrap();

            // TODO: Handle the file content and set clipboard
            // Need to think how avoid cycling through (Updating clipboard -> Uploading -> Downloading)
            println!("File content: {}", file_content);
        }

        if let Some(backoff) = result.backoff {
            std::thread::sleep(std::time::Duration::from_secs(backoff as u64));
        }
    }
}
