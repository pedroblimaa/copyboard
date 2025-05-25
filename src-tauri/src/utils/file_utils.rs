use std::{env, error::Error, fs::{self, File}, io::{self, Write}, path::PathBuf};

use crate::models::token::TokenData;

const TOKEN_FILE: &str = "token.json";

pub fn create_config_if_needed() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let config_dir = config_dir.join("copyboard");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).unwrap();
    }

    config_dir
}

pub fn save_token_to_file(token: &TokenData) {
    let json: String = serde_json::to_string(token).unwrap();
    let config_dir = create_config_if_needed();
    fs::write(config_dir.join(TOKEN_FILE), json).unwrap();
}

pub fn get_token_from_file() -> Result<TokenData, Box<dyn Error>> {
    let config_dir = create_config_if_needed();

    let config_file = config_dir
        .join(TOKEN_FILE);

    let file: String = fs::read_to_string(config_file)?;
    let token: TokenData = serde_json::from_str(&file)?;

    Ok(token)
}

pub fn load_html() -> Result<String, Box<dyn Error>> {
    let path = PathBuf::from("html/oauth_success.html");

    Ok(fs::read_to_string(&path).unwrap())
}



pub fn create_temp_file(content: &str) -> Result<File, io::Error> {
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("clipboard.txt");

    let mut file = File::create(&temp_file_path)?;
    file.write_all(content.as_bytes())?;

    let opened_file = File::open(temp_file_path)?;

    Ok(opened_file)
}
