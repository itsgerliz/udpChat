use std::net::UdpSocket;
use std::process::exit;
use log::{error, info, debug};
use crate::{HEADER_SIZE, HEADER_MAGIC, HEADER_VERSION, HEADER_MSGT_LOGIN};

pub(crate) fn init(target: &(&str, u16)) {
	let socket = match UdpSocket::bind(("127.0.0.1", 0)) {
		Ok(sock) => {
			info!("Socket up!");
			sock
		}
		Err(err) => {
			error!("Could not create UDP socket, OS sent: {}", err);
			exit(1);
		}
	};

	match send_login(&socket, target) {
		true => info!("OK"),
		false => error!("Not ok")
	}
}

fn send_login(socket: &UdpSocket, endpoint: &(&str, u16)) -> bool {
	// Compound header to send
	let mut header: [u8; HEADER_SIZE as usize] = [0; HEADER_SIZE as usize];
	header[..HEADER_MAGIC.len()].copy_from_slice(HEADER_MAGIC);
	header[HEADER_MAGIC.len()] = HEADER_VERSION;
	header[HEADER_MAGIC.len() + 1] = HEADER_MSGT_LOGIN;

	match socket.send_to(&header, *endpoint) {
		Ok(bsent) => {
			assert_eq!(bsent, header.len());
			debug!("Sent header to {}:{}", (*endpoint).0, (*endpoint).1);
			true
		}
		Err(err) => {
			error!("Could not send header to specified address, OS sent: {}", err);
			exit(1)
		}
	}
}