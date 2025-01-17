use std::net::UdpSocket;
use std::process::exit;
use log::{error, info, debug};
use crate::server::{HEADER_MAGIC, HEADER_MSGT_LOGIN, HEADER_SIZE, HEADER_VERSION};

pub(crate) fn init(target: &(&str, u16)) {
	let socket = match UdpSocket::bind(*target) {
		Ok(sock) => {
			info!("Socket up!");
			sock
		}
		Err(err) => {
			error!("Could not create UDP socket, OS sent: {}", err);
			exit(1);
		}
	};

	loop {
		todo!()
	}
}

fn send_login(socket: &UdpSocket, endpoint: &(&str, u16)) {
	// Compound header to send
	let mut header: [u8; HEADER_SIZE as usize] = [0; HEADER_SIZE as usize];
	header[..HEADER_MAGIC.len()].copy_from_slice(HEADER_MAGIC);
	header[HEADER_MAGIC.len()] = HEADER_VERSION;
	header[HEADER_MAGIC.len() + 1] = HEADER_MSGT_LOGIN;

	match socket.send_to(&header, endpoint) {
		Ok(_) => {
			todo!()
		}
		Err(err) => {
			todo!()
		}
	}
}