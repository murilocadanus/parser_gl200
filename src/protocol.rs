use std::str;
use std::io::BufReader;
use rustc_serialize::hex::ToHex;
use tpms::*;

#[derive(PartialEq,Eq,Debug)]
enum Message {
	LogOn 	= 0x10,
	Frame 	= 0x20,
	LogOff 	= 0xF0
}

pub struct Protocol {

}

impl Protocol {
	pub fn new() -> Protocol {
		Protocol {}
	}

	pub fn decode_buffer(&mut self, buffer: &Vec<u8>) {
		let service_id = buffer[2];
		let hex_buffer = &buffer[..].to_hex();

		//let logon = log_on(&hex_buffer).unwrap();

		match service_id {
			s if s == Message::LogOn as u8			=> debug!("Logon: {:?}", hex_buffer),
			s if s == Message::Frame as u8			=> debug!("Frame: {:?}", hex_buffer),
			s if s == Message::LogOff as u8			=> debug!("Logoff: {:?}", hex_buffer),
			_										=> debug!("Unknown service id"),
		}

	}
}