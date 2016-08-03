extern crate mio;
extern crate psio;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;

use psio::server::*;

fn main() {
	env_logger::init().ok().expect("Failed to init logger");

	let addr = "127.0.0.1:21012".parse::<SocketAddr>()
		.ok().expect("Failed to parse host:port string");

	let sock = TcpListener::bind(&addr).ok().expect("Failed to bind address");

	let mut event_loop = EventLoop::new().ok().expect("Failed to create event loop");

	let mut server = Server::new(sock);
	server.register(&mut event_loop).ok().expect("Failed to register server with event loop");

	info!("Even loop starting...");
	event_loop.run(&mut server).unwrap_or_else(|e| {
		error!("Event loop failed {:?}", e);
	});
}
