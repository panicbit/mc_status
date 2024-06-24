pub mod cli;
pub mod protocol;

use std::io::Write;
use std::net::TcpStream;

use anyhow::{Context, Result};
pub use cli::Cli;

use protocol::StatusResponse;

pub fn get_server_status(host: &str, port: u16) -> Result<StatusResponse> {
    let stream = &mut TcpStream::connect((host, port)).context("failed to connect")?;

    let protocol_version = -1;
    let next_state = 1;

    protocol::write_handshake(stream, protocol_version, host, port, next_state)?;
    protocol::write_status_request(stream)?;
    stream.flush()?;

    let status_response = protocol::read_status_response(stream)?;

    Ok(status_response)
}
