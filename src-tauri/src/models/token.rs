use serde::{Deserialize, Serialize};

// These should be moved to some file that stores structs (like modes in JS)
#[derive(Serialize, Deserialize, Default)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
}