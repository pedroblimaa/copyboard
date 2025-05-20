use crate::{
    config::config::{CLIENT_ID, DROPBOX_AUTH_URL, SERVER_URL},
    models::token::{TokenData, TokenResponse},
};

pub fn build_dropbox_auth_url() -> String {
    let encoded_redirect_uri = urlencoding::encode(SERVER_URL);

    format!(
        "{}&client_id={}&redirect_uri={}&token_access_type=offline",
        DROPBOX_AUTH_URL, CLIENT_ID, encoded_redirect_uri
    )
}

pub fn adapt_token_response(token_response: TokenResponse) -> TokenData {
    let time_now:u64 = chrono::Utc::now().timestamp() as u64;
    let expires_at = time_now + token_response.expires_in.unwrap_or_default();

    TokenData {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token.unwrap_or_default(),
        expires_at: expires_at.to_string(),
    }
}

pub fn get_code_from_url(url: &str) -> String {
    let query = url.split('?').nth(1).unwrap_or("");

    let code = query
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|c| c.trim_start_matches("code="))
        .unwrap_or("");

    code.to_string()
}
