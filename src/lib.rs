#![feature(async_closure)]

pub mod cli;
pub mod config;
pub mod output;
pub mod protocol;

use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

use anyhow::{bail, Context, Result};
pub use cli::Cli;
pub use config::Config;
use tokio::net::TcpStream;

use protocol::StatusResponse;
use tokio::time;

const APP_NAME: &str = "mc_status";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);

pub async fn get_server_status(host: &str, port: u16) -> Result<StatusResponse> {
    get_server_status_with_timeout(host, port, DEFAULT_TIMEOUT).await
}

pub async fn get_server_status_with_timeout(
    host: &str,
    port: u16,
    duration: Duration,
) -> Result<StatusResponse> {
    let status_response_future = get_server_status_without_timeout(host, port);
    let status_response_future = time::timeout(duration, status_response_future);

    let Ok(status_response) = status_response_future.await else {
        bail!("timed out after {duration:?}");
    };

    status_response
}

async fn get_server_status_without_timeout(host: &str, port: u16) -> Result<StatusResponse> {
    let addr = &(host, port).to_socket_addrs()?.next().unwrap();
    let stream = &mut TcpStream::connect(addr)
        .await
        .context("failed to connect")?;

    let protocol_version = -1;
    let next_state = 1;

    protocol::write_handshake(stream, protocol_version, host, port, next_state).await?;
    protocol::write_status_request(stream).await?;
    stream.flush().await?;

    let status_response = protocol::read_status_response(stream).await?;

    Ok(status_response)
}
