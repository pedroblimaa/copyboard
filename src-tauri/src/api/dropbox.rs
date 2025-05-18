use std::fs::File;

use reqwest::Error;

use crate::{
    config::config::{CLIEND_SECRET, CLIENT_ID, SERVER_URL},
    models::token::TokenResponse,
};

pub fn create_file(access_token: &str, file: File) {
    let client = reqwest::blocking::Client::new();

    client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header("Authorization", format!("Bearer {}", access_token))
        .header(
            "Dropbox-API-Arg",
            r#"{"path":"/test.txt","mode":"add","autorename":true,"mute":false}"#,
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
