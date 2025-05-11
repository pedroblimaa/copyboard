// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
use clipboard::start_clipboard_watcher;

fn main() {
    start_clipboard_watcher();

    copyboard_lib::run()
}
