use std::{error::Error, io::Cursor, thread};
use tauri::App;
use tauri_plugin_opener::OpenerExt;
use tiny_http::{Request, Response, Server};

use crate::{adapters::auth_adapter::AuthAdapter, api::dropbox, utils::file_utils};

pub fn handle_auth(app: &App) {
    let _ = match get_token() {
        Ok(_token) => return,
        Err(_) => {}
    };

    let auth_url: String = AuthAdapter::build_dropbox_auth_url();

    listen_for_auth_code().unwrap();
    app.opener().open_path(auth_url, None::<&str>).unwrap();
}

pub fn get_token() -> Result<String, Box<dyn Error>> {
    let token = file_utils::get_token_from_file()?;
    let expires_at: i64 = token.expires_at.parse()?;
    let refresh_token = token.refresh_token;

    let time_now = chrono::Utc::now().timestamp();

    if expires_at > time_now {
        return Ok(token.access_token);
    };

    let token_response = dropbox::refresh_token(refresh_token)?;
    let token_data = AuthAdapter::adapt_token_response(token_response);
    file_utils::save_token_to_file(&token_data);

    Ok(token_data.access_token)
}

fn listen_for_auth_code() -> Result<(), Box<dyn Error>> {
    let html = file_utils::load_html()?;
    let server = Server::http("0.0.0.0:53682").unwrap();

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let code_option = handle_request_listening(request, html.clone());
            if let Some(code) = code_option {
                log_in(code).unwrap();
                break;
            }
        }
    });

    Ok(())
}

fn handle_request_listening(request: Request, html: String) -> Option<String> {
    let url = request.url();

    if url.contains("code=") {
        let code = AuthAdapter::get_code_from_url(url);
        let html_response = get_html_response(html.clone());
        let _ = request.respond(html_response);

        return Some(code);
    }

    None
}

fn get_html_response(html: String) -> Response<Cursor<Vec<u8>>> {
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();

    Response::from_string(html)
        .with_status_code(200)
        .with_header(header)
}

fn log_in(code: String) -> Result<(), Box<dyn Error>> {
    let token_response = dropbox::login(code)?;

    let token_data = AuthAdapter::adapt_token_response(token_response);
    file_utils::save_token_to_file(&token_data);

    Ok(())
}
