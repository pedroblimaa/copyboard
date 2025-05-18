use serde::{Deserialize, Serialize};
use std::{error::Error, fs, io::Cursor, path::PathBuf, thread};
use tauri::App;
use tauri_plugin_opener::OpenerExt;
use tiny_http::{Request, Response, Server};

const DROPBOX_AUTH_URL: &str = "https://www.dropbox.com/oauth2/authorize?response_type=code";
const CLIENT_ID: &str = "g4fxyhqj8y7ajbd";
const CLIEND_SECRET: &str = "reb0r83pfq4ow99";
const SERVER_URL: &str = "http://localhost:53682";

// These should be moved to some file that stores structs (like modes in JS)
#[derive(Serialize, Deserialize)]
struct TokenData {
    access_token: String,
    refresh_token: String,
    expires_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}
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

    url
}

fn listen_for_auth_code() -> Result<(), Box<dyn Error>> {
    let html = load_html()?;
    let server = Server::http("0.0.0.0:53682").unwrap();

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let code_option = handle_request_listening(request, html.clone());
            if let Some(code) = code_option {
                println!("Code: {}", code);
                log_in(code).unwrap();
                push_test_file();

                break;
            }
        }
    });

    Ok(())
}

fn load_html() -> Result<String, Box<dyn Error>> {
    let path = PathBuf::from("html/oauth_success.html");

    Ok(fs::read_to_string(&path).unwrap())
}

fn handle_request_listening(request: Request, html: String) -> Option<String> {
    let url = request.url();

    if url.contains("code=") {
        let code = get_code_from_url(url);
        let html_response = get_html_response(html.clone());
        let _ = request.respond(html_response);

        return Some(code);
    }

    None
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

fn log_in(code: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .post("https://api.dropboxapi.com/oauth2/token")
        // TODO: This should be moved to an adapter file to adapt the requests and responses
        .form(&[
            ("code", code),
            ("grant_type", "authorization_code".to_string()),
            ("client_id", CLIENT_ID.to_string()),
            ("client_secret", CLIEND_SECRET.to_string()),
            ("redirect_uri", SERVER_URL.to_string()),
        ])
        .send();

    let token_response: TokenResponse = res?.json()?;
    let token_data = adapt_token_response(token_response);
    save_token_to_file(token_data)?;

    Ok(())
}

fn save_token_to_file(token: TokenData) -> Result<(), Box<dyn Error>> {
    let config_dir = dirs::config_dir().ok_or("Unable to get config directory")?;
    // TODO: This should be moved to a config, to be easier to read when needed
    let config_dir = config_dir.join("copyboard");
    println!("Config directory: {:?}", config_dir);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let json = serde_json::to_string(&token)?;

    fs::write(config_dir.join("token.json"), json).unwrap();

    Ok(())
}

fn adapt_token_response(token_response: TokenResponse) -> TokenData {
    TokenData {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token.unwrap_or_default(),
        expires_at: token_response.expires_in.unwrap_or_default().to_string(),
    }
}

// TODO: This should be removed, just a test push file 
fn push_test_file() {
    let config_file = dirs::config_dir()
        .unwrap()
        .join("copyboard")
        .join("token.json");

    let file = fs::read_to_string(config_file).unwrap();
    let token: TokenData = serde_json::from_str(&file).unwrap();

    let client = reqwest::blocking::Client::new();

    let file_path = PathBuf::from("test.txt");
    let file = fs::File::open(file_path).unwrap();

    let res = client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .header("Dropbox-API-Arg", r#"{"path":"/test.txt","mode":"add","autorename":true,"mute":false}"#)
        .header("Content-Type", "application/octet-stream")
        .body(file)
        .send()
        .unwrap();

    println!("Response: {:?}", res);
}