use std::net::{UdpSocket, SocketAddr};
use std::process::exit;
use log::{error, warn, info, debug};

const HEADER_SIZE: u8 = 9;
const HEADER_MAGIC: &[u8] = "udpChat".as_bytes();
const HEADER_VERSION: u16 = 100;
const HEADER_MSGT_LOGIN: u8 = 0x01;
const HEADER_MSGT_LOGOUT: u8 = 0x02;

struct Client {
	id: u8,
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

	let mut next_id: u8 = 1;

	loop {
		todo!()
	}
}

/*
fn check(socket: &UdpSocket, next_id: u8) -> Option<Client> {
	let mut buffer: [u8; 14] = [0; 14];
	match socket.recv_from(&mut buffer) {
		Ok(peer) => {
			info!("Incoming posibly valid packet from {}", peer.1);
			debug!("Received magic: {:?}", &buffer[0..=6]);
			debug!("Expected magic:{:?}", HEADER_MAGIC);
			debug!("Received version: {:?}", &buffer[7..=12]);
			debug!("Expected version: {:?}", UDPCHAT_VER);
			debug!("Received terminator: {:?}", &buffer[13]);
			debug!("Expected terminator: {:?}", b'\n');
			if buffer[0..=6] == *HEADER_MAGIC && buffer[7..=12] == *UDPCHAT_VER && buffer[13] == b'\n' {
				info!("Header is valid, client ID: {}", next_id);
				Option::Some(Client {
					id: next_id,
					inner: peer.1
				})
			} else {
				warn!("Invalid header, dropping...");
				Option::None
			}
		}
		Err(err) => {
			match err.raw_os_error() {
				// Windows error code for too long for buffer packets
				Option::Some(10040) => {
					warn!("Strange sent too long packet, dropping...");
					Option::None
				}
				_ => {
					error!("Could not read from socket, OS sent: {}", err);
					exit(1);
				}
			}
		}
	}
}
*/