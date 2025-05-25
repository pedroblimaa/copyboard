pub mod adapters;
pub mod api;
mod commands;
pub mod config;
pub mod models;
pub mod services;
pub mod utils;

use crate::config::config::AppConfig;
use std::sync::{Arc, Mutex};

use arboard::Clipboard;
use models::clipboard::StartClipboardWatcherInfo;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, Builder, Manager, Wry,
};

use commands::greet;
use services::clipboard_service;
use services::{auth_service, cloud_service};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = AppConfig::init();

    Builder::default()
        .manage(config)
        .plugin(tauri_plugin_shell::init())
        .setup(|app| setup_app(app))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    setup_tray(app)?;

    auth_service::handle_auth(app);
    setup_clipboard();

    Ok(())
}

fn setup_clipboard() {
    let clipboard_watcher_info = StartClipboardWatcherInfo {
        clipboard: Arc::new(Mutex::new(Clipboard::new().unwrap())),
        last_clipboard: Arc::new(Mutex::new(String::new())),
        file_pushed: Arc::new(Mutex::new(false)),
    };

    clipboard_service::start_clipboard_watcher(&clipboard_watcher_info);
    cloud_service::start_dropbox_file_watch(clipboard_watcher_info);
}

fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let tray_icon = app.default_window_icon().unwrap().clone();
    let menu = create_menu(app)?;

    TrayIconBuilder::new()
        .icon(tray_icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| handle_menu_events(app, event))
        .build(app)?;

    Ok(())
}

fn create_menu(app: &App) -> Result<Menu<Wry>, tauri::Error> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;

    Menu::with_items(app, &[&quit_i, &open_i, &hide_i])
}

fn handle_menu_events(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "quit" => app.exit(0),
        "open" => app.get_webview_window("main").unwrap().show().unwrap(),
        "hide" => app.get_webview_window("main").unwrap().hide().unwrap(),
        _ => {}
    }
}
