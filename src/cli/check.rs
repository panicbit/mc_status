use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::{Config, output};

#[derive(Serialize, Deserialize, Default, Clone, clap::Args)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(clap::Args, Clone)]
pub struct Cli {}

impl Cli {
    pub fn run(self) -> Result<()> {
        let config = Config::load()?;
        let mut all_responses = Vec::new();
        for server in &config.server_list {
            let server_status = crate::get_server_status(&server.host, server.port).context("failed to get server status");
            match server_status {
                Ok(server_status) => {all_responses.push((server_status, server))}
                Err(err) => {println!("{:?}", err)}
            }
        }
        output::display_all_responses(all_responses);
        Ok(())
    }
}
