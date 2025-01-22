use clap::Parser;
use log::{info, debug};

mod client;
mod server;

pub(crate) const HEADER_SIZE: u8 = 9;
pub(crate) const HEADER_TOTAL_SIZE: u16 = 512;
pub(crate) const HEADER_MAGIC: &[u8] = "udpChat".as_bytes();
pub(crate) const HEADER_VERSION: u8 = 0x1;
pub(crate) const HEADER_MSGT_LOGIN: u8 = 0x1;
pub(crate) const HEADER_MSGT_LOGOUT: u8 = 0x2;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "Run as server?", short = 'l', long = "listen", required = false)]
    server: bool,
    #[arg(help = "Where should we connect/listen?", required = true)]
    target: String,
    #[arg(help = "Port to connect/listen to/on", required = true)]
    target_port: u16
}

fn main() {
    let args = CliArgs::parse();
    
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    debug!("Initiated logger!");

    let target_addr = (args.target.as_str(), args.target_port);

    if args.server == true {
        info!("Initiating server...");
        server::init(&target_addr);
    } else {
        info!("Initiating client...");
        client::init(&target_addr);
    }
}