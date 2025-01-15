use clap::Parser;
use log::{error, warn, info, debug, trace};

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "IP address to connect to", required = true)]
    target: String,
    #[arg(help = "Port to connect to", required = true)]
    target_port: u16
}

fn main() {
    let args = CliArgs::parse();
    env_logger::init();
}
