#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]

use clap::Parser;
use std::env;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// ircd, a simple IRC server implementation.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug logging
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

    // ...

    let address = format!("{}:{}", cli.host, cli.port);
    let listener = TcpListener::bind(&address).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // ...
    }
}

