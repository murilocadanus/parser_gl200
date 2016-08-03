extern crate byteorder;
extern crate rustc_serialize;

use rustc_serialize::hex::FromHex;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

use byteorder::{ByteOrder, BigEndian};

static NTHREADS: i32 = 1;

fn main() {

	send_log_in();
/*
	for i in 0..NTHREADS {

		let _ = thread::spawn(move|| {

			let mut stream = TcpStream::connect("127.0.0.1:21012").unwrap();

			loop {
				let hex = "0F001015A0D7130111433141020E1E".from_hex().unwrap();
				//let msg = format!("{}", hex);
				let msg = hex;
				let mut buf = [0u8; 8];

				println!("thread {}: Sending over message length of {}", i, msg.len());
				BigEndian::write_u64(&mut buf, msg.len() as u64);
				stream.write_all(buf.as_ref()).unwrap();
				stream.write_all(msg.as_ref()).unwrap();

				let mut buf = [0u8; 8];
				stream.read(&mut buf).unwrap();

				let msg_len = BigEndian::read_u64(&mut buf);
				println!("thread {}: Reading message length of {}", i, msg_len);

				let mut r = [0u8; 256];
				let s_ref = <TcpStream as Read>::by_ref(&mut stream);

				match s_ref.take(msg_len).read(&mut r) {
					Ok(0) => {
						println!("thread {}: 0 bytes read", i);
					},
					Ok(n) => {
						println!("thread {}: {} bytes read", i, n);

						let s = std::str::from_utf8(&r[..]).unwrap();
						println!("thread {} read = {}", i, s);
					},
					Err(e) => {
						panic!("thread {}: {}", i, e);
					}
				}
			}
		});
	}

	loop {}
*/
}

fn send_log_in() {
	let mut stream = TcpStream::connect("127.0.0.1:21012").unwrap();
	let hex = "0F001015A0D7130111433141020E1E".from_hex().unwrap();
	let msg = hex;
	let mut buf = [0u8; 8];

	BigEndian::write_u64(&mut buf, msg.len() as u64);
	stream.write_all(buf.as_ref()).unwrap();
	stream.write_all(msg.as_ref()).unwrap();

	let mut buf = [0u8; 8];
	stream.read(&mut buf).unwrap();

	/*
	let msg_len = BigEndian::read_u64(&mut buf);
	println!("Reading message length of {}", msg_len);

	let mut r = [0u8; 256];
	let s_ref = <TcpStream as Read>::by_ref(&mut stream);

	match s_ref.take(msg_len).read(&mut r) {
		Ok(0) => {
			println!("0 bytes read");
		},
		Ok(n) => {
			println!("{} bytes read", n);

			let s = std::str::from_utf8(&r[..]).unwrap();
			println!("read = {}", s);
		},
		Err(e) => {
			panic!("{}", e);
		}
	}*/

}