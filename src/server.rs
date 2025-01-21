use std::net::{UdpSocket, SocketAddr};
use std::process::exit;
use log::{error, info, debug};
use crate::{HEADER_MAGIC, HEADER_VERSION, HEADER_MSGT_LOGIN};

struct Client {
	inner: SocketAddr
}

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
		match accept(&socket) {
			Some(client) => info!("Client registered"),
			None => continue
		}
	}
}

fn accept(socket: &UdpSocket) -> Option<Client> {
	let mut buffer: [u8; 512] = [0; 512];
	match socket.recv_from(&mut buffer) {
		Ok(peer) => {
			info!("Incoming connection from {}", peer.1);
			debug!("Checking header...");

			if 
				buffer[0..=6] == *HEADER_MAGIC &&
				buffer[7] == HEADER_VERSION &&
				buffer[8] == HEADER_MSGT_LOGIN
			{
				Some(Client {
					inner: peer.1
				})
			}
			else if
				buffer[0..=6] == *HEADER_MAGIC &&
				buffer[7] == HEADER_VERSION &&
				buffer[8] != HEADER_MSGT_LOGIN
			{
				info!("Client sent invalid operation, connection dropped");
				None
			}
			else {
				info!("Invalid header, connection dropped");
				None
			}
		}
		Err(err) => {
			match err.raw_os_error() {
				// Windows error code for too long for buffer packets
				Some(10040) => {
					debug!("Strange sent too long packet, connection dropped");
					None
				}
				// Any other error will terminate the process, server is unusable
				_ => {
					error!("Could not read from socket, OS sent: {}", err);
					exit(1);
				}
			}
		}
	}
}