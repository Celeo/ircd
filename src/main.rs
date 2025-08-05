use clap::Parser;
use log::{debug, info};
use std::env;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// ircd, a simple IRC server implementation.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug logging to a 'vatsim_online.log' file
    #[clap(short, long)]
    debug: bool,

    #[clap(long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = 8000)]
    port: u16,
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

    let address = format!("{}:{}", cli.host, cli.port);
    let listener = TcpListener::bind(&address).await.unwrap();
    info!("Listening on {address}");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process_socket(socket).await;
    }
}

async fn process_socket(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];
    loop {
        let n = socket.read(&mut buf).await.unwrap();
        if n == 0 {
            info!("Read 0 bytes; disconnecting client");
            break;
        }
        let read = &buf[0..n];
        let read = std::str::from_utf8(read).unwrap();
        if read.len() > 2 {
            info!(": {}", read.trim());
            _ = socket.write(b"I hear you\n").await.unwrap();
        }
    }
}
