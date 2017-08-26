extern crate env_logger;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate juniper;
extern crate juniper_iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;

use hyper::net::{HttpListener, NetworkListener};
use graph::Root;
use iron::prelude::*;
use iron::Protocol;
use juniper::EmptyMutation;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;
use std::env;
use std::os::unix::io::FromRawFd;

mod graph;

fn context_factory(_: &mut Request) -> Root {
    Root::new()
}

fn main() {
    let _logger = env_logger::init();
    debug!("Initium est");
    debug!("Statum quoque scribæ: {:?}", _logger);

    let mut mount = Mount::new();
    mount.mount("/", GraphiQLHandler::new("/graphql"));
    mount.mount("/graphql", GraphQLHandler::new(
        context_factory,
        Root::new(),
        EmptyMutation::<Root>::new()
    ));

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let mut listener = env::var("LISTEN_FD").ok()
        .and_then(|fd| fd.parse().ok())
        .and_then(|fd| Some(unsafe {
            HttpListener::from_raw_fd(fd)
        }))
        .unwrap_or_else(|| {
            let host = env::var("HOST").unwrap_or("0.0.0.0".into());
            let port = env::var("PORT").unwrap_or("8080".into());
            let addr = format!("{}:{}", host, port);
            HttpListener::new(addr).unwrap()
        });

    let netstr = listener.local_addr()
        .and_then(|a| Ok(format!("{}", a)))
        .unwrap_or("LISTEN_FD".into());

    info!("Quod est in Via Appia {}", netstr);
    Iron::new(chain).listen(listener, Protocol::http()).unwrap();
}
