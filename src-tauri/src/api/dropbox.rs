use std::fs::File;

use reqwest::Error;

use crate::{
    config::config::{CLIEND_SECRET, CLIENT_ID, SERVER_URL},
    models::token::TokenResponse,
};

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

pub fn login(code: String) -> Result<TokenResponse, Error> {
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
        .send();

    res?.json()
}

pub fn refresh_token(refresh_token: String) -> Result<TokenResponse, Error> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .post("https://api.dropboxapi.com/oauth2/token")
        .form(&[
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token".to_owned()),
            ("client_id", CLIENT_ID.to_owned()),
            ("client_secret", CLIEND_SECRET.to_owned()),
        ])
        .send();

    res?.json()
}
