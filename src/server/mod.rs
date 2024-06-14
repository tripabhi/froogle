use std::sync::{Arc, Mutex};

use opener::open_browser;
use tiny_http::Server;

use crate::model::Model;

pub mod routes;

pub fn start(address: &str, model: Arc<Mutex<Model>>) -> Result<(), ()> {
    let server = Server::http(address).map_err(|err| {
        log::error!("Cannot start http server at {address} : {err}");
    })?;

    log::info!("Access Froogle by visiting http://{address}");

    open_browser(format!("http://{address}").as_str()).ok();

    for request in server.incoming_requests() {
        routes::handle_request(request, Arc::clone(&model))
            .map_err(|err| {
                log::error!("Cannot respond to request {err}");
            })
            .ok();
    }

    log::error!("Server has shutdown");
    Err(())
}
