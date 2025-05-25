use std::sync::{Arc, Mutex};

use arboard::Clipboard;

pub type SharedClipboard = Arc<Mutex<Clipboard>>;
pub type SharedLastClip = Arc<Mutex<String>>;

pub struct StartClipboardWatcherInfo {
    pub clipboard: SharedClipboard,
    pub last_clipboard: SharedLastClip,
    pub file_pushed: Arc<Mutex<bool>>,
}
