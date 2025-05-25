use crate::{
    config::globals::CONFIG,
    models::token::{TokenData, TokenResponse},
};

pub struct AuthAdapter;

impl AuthAdapter {
    pub fn build_dropbox_auth_url() -> String {
        let encoded_redirect_uri = urlencoding::encode(&CONFIG.server_url);

        format!(
            "{}&client_id={}&redirect_uri={}&token_access_type=offline",
            &CONFIG.dropbox_auth_url, &CONFIG.client_id, encoded_redirect_uri
        )
    }

    pub fn adapt_token_response(token_response: TokenResponse) -> TokenData {
        let time_now: u64 = chrono::Utc::now().timestamp() as u64;
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
}
