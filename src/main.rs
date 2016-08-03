extern crate mio;
extern crate psio;

use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;

fn main() {
	let addr = "127.0.0.1:21012".parse::<SocketAddr>()
		.ok().expect("Failed to parse host:port string");

	let sock = TcpListener::bind(&addr).ok().expect("Failed to bind address");

	let mut event_loop = EventLoop::new().ok().expect("Failed to create event loop");

	let mut server = Server::new(sock);
	server.register(&mut event_loop).ok().expect("Failed to register server with event loop");

	println!("Even loop starting...");
	event_loop.run(&mut server).unwrap_or_else(|e| {
		println!("Event loop failed {:?}", e);
	});
}
