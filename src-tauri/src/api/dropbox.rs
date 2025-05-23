use std::fs::File;

use reqwest::blocking::Response;

use crate::{
    config::config::{CLIEND_SECRET, CLIENT_ID, SERVER_URL},
    models::{
        dropbox_file::{FolderResponse, LongpollResponse},
        token::TokenResponse,
    },
};

const FOLDER_PATH: &str = "";
const FILE_PATH: &str = "/clipboard.txt";

pub fn upload_file(access_token: String, file: File) {
    let client = reqwest::blocking::Client::new();

    client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header("Authorization", format!("Bearer {}", access_token))
        .header(
            "Dropbox-API-Arg",
            r#"{"path":"/clipboard.txt","mode":"overwrite","autorename":true,"mute":false}"#,
        )
        .header("Content-Type", "application/octet-stream")
        .body(file)
        .send()
        .unwrap();
}

pub fn login(code: String) -> Result<TokenResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .post("https://api.dropboxapi.com/oauth2/token")
        .form(&[
            ("code", code),
            ("grant_type", "authorization_code".to_owned()),
            ("client_id", CLIENT_ID.to_owned()),
            ("client_secret", CLIEND_SECRET.to_owned()),
            ("redirect_uri", SERVER_URL.to_owned()),
        ])
        .send()?;

    let res = handle_response_status(res);

    res.json()
}

pub fn refresh_token(refresh_token: String) -> Result<TokenResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .post("https://api.dropboxapi.com/oauth2/token")
        .form(&[
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token".to_owned()),
            ("client_id", CLIENT_ID.to_owned()),
            ("client_secret", CLIEND_SECRET.to_owned()),
        ])
        .send()?;

    let res = handle_response_status(res);

    res.json()
}

pub fn get_clipboard_file(access_token: String) -> Result<FolderResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let body = format!(r#"{{ "path": "{}" }}"#, FOLDER_PATH);
    let res = client
        .post("https://api.dropboxapi.com/2/files/list_folder")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    let res = handle_response_status(res);

    res.json()
}

pub fn longpoll(cursor: &str) -> Result<LongpollResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let body = serde_json::json!({
        "cursor": cursor,
        "timeout": 480
    });
    let body = body.to_string();

    let res = client
        .post("https://notify.dropboxapi.com/2/files/list_folder/longpoll")
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    let res = handle_response_status(res);

    res.json()
}

pub fn cursor_continue(token: &str, cursor: String) -> Result<FolderResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let body = format!(r#"{{ "cursor": "{}" }}"#, cursor);
    let res = client
        .post("https://api.dropboxapi.com/2/files/list_folder/continue")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    let res = handle_response_status(res);

    res.json()
}

pub fn download_file(token: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let body = format!(r#"{{ "path": "{}" }}"#, FILE_PATH);
    let res = client
        .post("https://content.dropboxapi.com/2/files/download")
        .header("Authorization", format!("Bearer {}", token))
        .header("Dropbox-API-Arg", body)
        .send()?;

    let res = handle_response_status(res);

    res.text()
}

pub fn handle_response_status(res: Response) -> Response {
    if res.status().is_success() {
        return res;
    }

    let status = res.status();
    let error_message = res.text().unwrap();
    println!("Error: {}", error_message);

    panic!("{}, {}", status, error_message);
}
