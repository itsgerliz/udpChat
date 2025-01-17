use clap::Parser;
use log::{info, debug};

mod client;
mod server;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "Run as server?", short = 'l', long = "listen", required = false)]
    server: bool,
    #[arg(help = "Where should we listen?", required = true)]
    target: String,
    #[arg(help = "Port to listen on", required = true)]
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