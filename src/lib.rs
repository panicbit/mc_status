pub mod cli;
pub mod config;
pub mod output;
pub mod protocol;

use tokio::io::{AsyncWriteExt};
use std::net::{ToSocketAddrs};

use anyhow::{Context, Result};
use tokio::net::TcpStream;
pub use cli::Cli;
pub use config::Config;

use protocol::StatusResponse;

const APP_NAME: &str = "mc_status";

pub async fn get_server_status(host: &str, port: u16) -> Result<StatusResponse> {
    let addr = &(host, port).to_socket_addrs()?.next().unwrap();
    let stream= &mut TcpStream::connect(addr).await
        .context("failed to connect")?;

    let protocol_version = -1;
    let next_state = 1;

    protocol::write_handshake(stream, protocol_version, host, port, next_state).await?;
    protocol::write_status_request(stream).await?;
    stream.flush().await?;

    let status_response = protocol::read_status_response(stream).await?;

    Ok(status_response)
}
