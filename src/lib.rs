pub mod cli;
pub mod config;
pub mod protocol;
pub mod output;

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use anyhow::{Context, Result};
pub use cli::Cli;
pub use config::Config;

use protocol::StatusResponse;

const APP_NAME: &str = "mc_status";

pub fn get_server_status(host: &str, port: u16) -> Result<StatusResponse> {
    let addr = &(host, port).to_socket_addrs()?.next().unwrap();
    let stream = &mut TcpStream::connect_timeout(addr, Duration::from_secs(3)).context("failed to connect")?;

    let protocol_version = -1;
    let next_state = 1;

    protocol::write_handshake(stream, protocol_version, host, port, next_state)?;
    protocol::write_status_request(stream)?;
    stream.flush()?;

    let status_response = protocol::read_status_response(stream)?;

    Ok(status_response)
}
