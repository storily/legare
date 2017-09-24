extern crate env_logger;
extern crate hyper;
extern crate iron;
extern crate iron_json_response;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate pest;
#[cfg(not(test))]
extern crate pest;

use hyper::net::{HttpListener, NetworkListener};
use iron::prelude::*;
use iron::Protocol;
use iron_json_response::JsonResponseMiddleware;
use logger::Logger;
use mount::Mount;
use std::env;
use std::os::unix::io::FromRawFd;

mod handler;
mod parse;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{}=info", env!("CARGO_PKG_NAME")));
    }

    let _logger = env_logger::init();
    info!("Starting up");
    debug!("Logger status: {:?}", _logger);

    let mut mount = Mount::new();
    info!("Mounting /");
    mount.mount("/", handler::parse);

    debug!("Making iron chain");
    let mut chain = Chain::new(mount);
    chain.link_after(JsonResponseMiddleware::new());

    debug!("Mounting request logger");
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    debug!("Finding socket");
    let mut listener = env::var("LISTEN_FD")
        .ok()
        .and_then(|fd| fd.parse().ok())
        .and_then(|fd| {
            info!("Found LISTEN_FD, binding to that socket");
            Some(unsafe { HttpListener::from_raw_fd(fd) })
        })
        .unwrap_or_else(|| {
            info!("No LISTEN_FD, creating a socket ourselves");
            let host = env::var("HOST").unwrap_or("0.0.0.0".into());
            let port = env::var("PORT").unwrap_or("8080".into());
            let addr = format!("{}:{}", host, port);
            HttpListener::new(addr).unwrap()
        });

    let netstr = listener
        .local_addr()
        .and_then(|a| Ok(format!("{}", a)))
        .unwrap_or("LISTEN_FD".into());

    info!("Legare ready and able at {}", netstr);
    Iron::new(chain).listen(listener, Protocol::http()).unwrap();
}
