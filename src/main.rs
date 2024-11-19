use clap::Parser;
use log::debug;
use std::env;

/// ircd, a simple IRC server implementation.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug logging to a 'vatsim_online.log' file
    #[clap(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.debug {
        env::set_var("RUST_LOG", "debug");
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    debug!("Starting");
}
