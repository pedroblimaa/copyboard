use std::{error::Error, fs, io::Cursor, path::PathBuf, thread};
use tauri::App;
use tauri_plugin_opener::OpenerExt;
use tiny_http::{Response, Server};

const DROPBOX_AUTH_URL: &str = "https://www.dropbox.com/oauth2/authorize?response_type=code";
const CLIENT_ID: &str = "g4fxyhqj8y7ajbd";

pub fn open_dropbox_auth_page(app: &App) {
    let auth_url = build_dropbox_auth_url(CLIENT_ID, "http://localhost:53682");

    listen_for_auth_code().unwrap();
    app.opener().open_path(auth_url, None::<&str>).unwrap();
}

fn build_dropbox_auth_url(app_key: &str, redirect_uri: &str) -> String {
    let encoded_redirect_uri = urlencoding::encode(redirect_uri);

    let url = format!(
        "{}&client_id={}&redirect_uri={}",
        DROPBOX_AUTH_URL, app_key, encoded_redirect_uri
    );

    println!("Dropbox Auth URL: {}", url);

    url
}

fn listen_for_auth_code() -> Result<(), Box<dyn Error>> {
    let html = load_html()?;
    let server = Server::http("0.0.0.0:53682").unwrap();

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let url = request.url();
            if url.contains("code=") {
                let code = get_code_from_url(url);

                println!("🔐 Received code: {}", code);

                let html_response = get_html_response(html.clone());
                let _ = request.respond(html_response);
                break;
            }
        }
    });

    Ok(())
}

fn load_html() -> Result<String, Box<dyn Error>> {
    let path = PathBuf::from("html/oauth_success.html");
    println!("Path: {:?}", path);
    println!("Current directory: {:?}", std::env::current_dir());
    let html = fs::read_to_string(&path).unwrap_or_else(|e| {
        println!("Error reading HTML file: {}", e);
        String::new()
    });

    Ok(html)
}

fn get_code_from_url(url: &str) -> String {
    let query = url.split('?').nth(1).unwrap_or("");

    let code = query
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|c| c.trim_start_matches("code="))
        .unwrap_or("");

    code.to_string()
}

fn get_html_response(html: String) -> Response<Cursor<Vec<u8>>> {
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();

    Response::from_string(html)
        .with_status_code(200)
        .with_header(header)
}
