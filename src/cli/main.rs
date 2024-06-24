use std::net::TcpStream;

use anyhow::{Context, Result};

use crate::*;

#[derive(clap::Args)]
pub struct Cli {
    host: String,
    #[clap(default_value_t = 25565)]
    port: u16,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let host = self.host.as_str();
        let port = self.port;

        let stream = &mut TcpStream::connect((host, port)).context("failed to connect")?;

        let protocol_version = -1;
        let next_state = 1;

        write_handshake(stream, protocol_version, host, port, next_state)?;
        write_status_request(stream)?;
        stream.flush()?;

        let status_response = read_status_response(stream)?;

        let players = status_response.players;

        println!("{} player(s) online:", players.online);

        for player in &players.sample {
            println!("{}", player.name);
        }

        if players.online > players.sample.len() as i32 {
            println!("...");
        }

        Ok(())
    }
}
