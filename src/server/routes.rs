use std::fs::File;
use std::sync::{Arc, Mutex};

use tiny_http::{Header, Method, Request, Response, StatusCode};

use crate::model::Model;
use crate::util;
use std::path::PathBuf;

static CONTENT_TYPE_HEADER: &str = "Content-Type";
static JS_CONTENT_TYPE: &str = "text/javascript; charset=utf-8";
static CSS_CONTENT_TYPE: &str = "text/css; charset=utf-8";
static HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";
static TXT_CONTENT_TYPE: &str = "text/plain; charset=utf-8";
static JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";

static XML_CONTENT_TYPE: &str = "application/xhtml+xml; charset=utf-8";
static PDF_CONTENT_TYPE: &str = "application/pdf; charset=utf-8";

pub fn handle_request(request: Request, _model: Arc<Mutex<Model>>) -> std::io::Result<()> {
    match (request.method(), request.url()) {
        (Method::Get, "/") | (Method::Get, "/index.html") => serve_bytes(
            request,
            200,
            include_bytes!("../assets/index.html"),
            HTML_CONTENT_TYPE,
        ),
        (Method::Get, "/index.js") => serve_bytes(
            request,
            200,
            include_bytes!("../assets/index.min.js"),
            JS_CONTENT_TYPE,
        ),
        (Method::Get, "/styles.css") => serve_bytes(
            request,
            200,
            include_bytes!("../assets/styles.min.css"),
            CSS_CONTENT_TYPE,
        ),
        (Method::Get, path) => {
            let file_name = path.to_owned();
            serve_file(request, &file_name)
        }
        (Method::Post, "/api/search") => handle_search_api(request, Arc::clone(&_model)),
        _ => serve_http_error(request, 404, "Unknown request"),
    }
}

pub fn handle_search_api(mut request: Request, model: Arc<Mutex<Model>>) -> std::io::Result<()> {
    let mut body = Vec::new();
    if let Err(err) = request.as_reader().read_to_end(&mut body) {
        log::error!("Failed to read request body : {err}");
        return serve_http_error(request, 500, "Failed to read request body");
    }

    let query = match String::from_utf8(body) {
        Ok(query) => query,
        Err(err) => {
            log::error!("Failed to read request body as UTF-8 string : {err}");
            return serve_http_error(request, 400, "Request body is not valid UTF-8");
        }
    };

    let results: Vec<(PathBuf, f32)>;
    {
        let model = model.lock().unwrap();
        results = model.search_document(&query);
    }

    let json = match serde_json::to_string(
        &results
            .iter()
            .map(|(path, _rank)| {
                (
                    path,
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unknown encoding"),
                )
            })
            .take(15)
            .collect::<Vec<_>>(),
    ) {
        Ok(json) => json,
        Err(err) => {
            log::error!("Cannot serialize response as JSON : {err}");
            return serve_http_error(request, 500, "Failed to serialize response");
        }
    };

    request.respond(Response::from_string(json).with_header(
        Header::from_bytes(CONTENT_TYPE_HEADER, JSON_CONTENT_TYPE).expect("Invalid headers"),
    ))
}

pub fn serve_bytes(
    request: Request,
    status_code: u16,
    bytes: &[u8],
    content_type: &str,
) -> std::io::Result<()> {
    let content_type_header =
        Header::from_bytes(CONTENT_TYPE_HEADER, content_type).expect("Cannot create header");
    request.respond(
        Response::from_data(bytes)
            .with_status_code(StatusCode(status_code))
            .with_header(content_type_header),
    )
}

pub fn serve_str(request: Request, status_code: u16, content: &str) -> std::io::Result<()> {
    request.respond(Response::from_string(content).with_status_code(StatusCode(status_code)))
}

pub fn serve_file(request: Request, file_path: &str) -> std::io::Result<()> {
    let file = File::open(file_path);

    match file {
        Ok(file) => {
            let content_type = match util::file_extension(file_path) {
                Some(extension) => match extension.as_str() {
                    "xml" | "xhtml" => XML_CONTENT_TYPE,
                    "pdf" => PDF_CONTENT_TYPE,
                    "json" => JSON_CONTENT_TYPE,
                    _other => TXT_CONTENT_TYPE,
                },
                None => TXT_CONTENT_TYPE,
            };

            let content_type_header = Header::from_bytes(CONTENT_TYPE_HEADER, content_type)
                .expect("Cannot create header");

            request.respond(Response::from_file(file).with_header(content_type_header))
        }
        Err(err) => serve_http_error(request, 500, &err.to_string()),
    }
}

pub fn serve_http_error(request: Request, error_code: u16, message: &str) -> std::io::Result<()> {
    let status_code = StatusCode(error_code);
    let content = format!(
        "{code} {reason} : {message}",
        code = error_code,
        reason = status_code.default_reason_phrase()
    );

    serve_str(request, error_code, &content)
}
