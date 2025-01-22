use std::net::UdpSocket;
use std::process::exit;
use log::{error, info, debug};
use crate::{HEADER_SIZE, HEADER_MAGIC, HEADER_VERSION, HEADER_MSGT_LOGIN, HEADER_MSGT_LOGOUT};

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

	// Create the header here to avoid recreating it on each function that needs it
	let mut header: [u8; HEADER_SIZE as usize] = [0; HEADER_SIZE as usize];
	// Header magic and version are the same for the entire runtime
	header[..HEADER_MAGIC.len()].copy_from_slice(HEADER_MAGIC);
	header[HEADER_MAGIC.len()] = HEADER_VERSION;
	// Message type is initialized to 0 but must be filled by the function
	header[HEADER_MAGIC.len() + 1] = 0;
	// The message type will be filled by the function with the following:
	// header[HEADER_MAGIC.len() + 1] = <messagetype>;
	// The function must reset this to zero when done

	send_login(&socket, target, &mut header);
}

fn send_login(socket: &UdpSocket, endpoint: &(&str, u16), header: &mut [u8; HEADER_SIZE as usize]) {
	// Add message type
	(*header)[HEADER_MAGIC.len() + 1] = HEADER_MSGT_LOGIN;

	match socket.send_to(header, *endpoint) {
		Ok(bsent) => {
			assert_eq!(bsent, header.len());
			debug!("Sent header to {}:{}", (*endpoint).0, (*endpoint).1);
			// Reset message type field
			(*header)[HEADER_MAGIC.len() + 1] = 0
		}
		Err(err) => {
			error!("Could not send header to specified address, OS sent: {}", err);
			exit(1)
		}
	}
}

/*
fn await_response(socket: &UdpSocket, endpoint: &(&str, u16)) {

}
*/