use arboard::Clipboard;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn start_clipboard_watcher() {
    let clipboard = Arc::new(Mutex::new(Clipboard::new().unwrap()));
    let last_clipboard = Arc::new(Mutex::new(String::new()));

    let clipboard_clone = Arc::clone(&clipboard);
    let last_clipboard_clone = Arc::clone(&last_clipboard);
    // let app_handle = app.clone();

    thread::spawn(move || loop {
        let mut clipboard = clipboard_clone.lock().unwrap();
        if let Ok(current_text) = clipboard.get_text() {
            let mut last = last_clipboard_clone.lock().unwrap();
            if *last != current_text {
                *last = current_text.clone();
                println!("Clipboard changed: {}", current_text);
            }
        }

        thread::sleep(Duration::from_millis(500));
    });
}
 