use super::local_config::{DROPBOX_KEY, DROPBOX_SECRET};

const DROPBOX_AUTH_URL: &str = "https://www.dropbox.com/oauth2/authorize?response_type=code";
const SERVER_URL: &str = "http://localhost:53682";

pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub dropbox_auth_url: String,
    pub server_url: String,
}

impl AppConfig {
    pub fn init() -> Self {
        AppConfig {
            client_id: DROPBOX_KEY.to_string(),
            client_secret: DROPBOX_SECRET.to_string(),
            dropbox_auth_url: DROPBOX_AUTH_URL.to_string(),
            server_url: SERVER_URL.to_string(),
        }
    }
}
