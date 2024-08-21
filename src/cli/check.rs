use crate::{output, Config};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Serialize, Deserialize, Default, Clone, clap::Args)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(clap::Args, Clone)]
pub struct Cli {}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let config = Config::load()?;
        let mut all_responses = Vec::new();
        for server in &config.server_list {
            let server_status_future = crate::get_server_status(&server.host, server.port);
            let timeout_status = timeout(Duration::from_millis(1000), server_status_future).await;
            let server_status = match timeout_status {
                Ok(server_status) => server_status,
                Err(_err) => {
                    bail!("timed out!")
                }
            };
            match server_status {
                Ok(server_status) => all_responses.push((server_status, server)),
                Err(err) => return Err(err),
            }
        }
        output::display_all_responses(all_responses);
        Ok(())
    }
}
