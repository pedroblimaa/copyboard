use std::{error::Error, fs, io::Cursor, path::PathBuf, thread};
use tauri::App;
use tauri_plugin_opener::OpenerExt;
use tiny_http::{Request, Response, Server};

use crate::{adapters::auth_adapter, api::dropbox, utils::file_utils};

pub fn handle_auth(app: &App) {
    let auth_url: String = auth_adapter::build_dropbox_auth_url();

    listen_for_auth_code().unwrap();
    app.opener().open_path(auth_url, None::<&str>).unwrap();
}

fn listen_for_auth_code() -> Result<(), Box<dyn Error>> {
    let html = file_utils::load_html()?;
    let server = Server::http("0.0.0.0:53682").unwrap();

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let code_option = handle_request_listening(request, html.clone());
            if let Some(code) = code_option {
                println!("Code: {}", code);
                log_in(code).unwrap();
                push_test_file();

                break;
            }
        }
    });

    Ok(())
}

fn handle_request_listening(request: Request, html: String) -> Option<String> {
    let url = request.url();

    if url.contains("code=") {
        let code = auth_adapter::get_code_from_url(url);
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

    let token_data = auth_adapter::adapt_token_response(token_response);
    file_utils::save_token_to_file(token_data);

    Ok(())
}

// TODO: This should be removed, just a test push file
fn push_test_file() {
    let token = file_utils::get_token_from_file().unwrap();

    let file_path = PathBuf::from("test.txt");
    let file = fs::File::open(file_path).unwrap();

    dropbox::create_file(&token.access_token, file);
}
