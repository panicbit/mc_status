use crate::{get_server_status, output, Config};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

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
            let server_status = get_server_status(&server.host, server.port)
                .await
                .context("failed to get server status")?;

            all_responses.push((server_status, server));
        }

        output::display_all_responses(all_responses);

        Ok(())
    }
}
