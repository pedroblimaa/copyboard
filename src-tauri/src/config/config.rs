use core::panic;
use std::env;

use dotenv::dotenv;

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
        dotenv().ok();

        let client_id = env::var("DROPBOX_KEY").unwrap_or_else(|_| Self::handle_env_err());
        let client_secret = env::var("DROPBOX_SECRET").unwrap_or_else(|_| Self::handle_env_err());

        AppConfig {
            client_id,
            client_secret,
            dropbox_auth_url: DROPBOX_AUTH_URL.to_string(),
            server_url: SERVER_URL.to_string(),
        }
    }

    fn handle_env_err() -> String {
        panic!("Environment variable not found, please add DROPBOX_KEY and DROPBOX_SECRET to .env file.");
    }
}
