use std::time::Duration;
use crate::{config, output, Config};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

#[derive(Serialize, Deserialize, Default, Clone, clap::Args)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub alias: String,
}

#[derive(clap::Args, Clone)]
pub struct Cli {
    #[clap(flatten)]
    pub server: Server,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let mut config = Config::load()?;
        let server = config::Server::new(self.server.host.clone(), self.server.port, self.server.alias);
        
        config.server_list.insert(server.clone());
        let server_status_future = crate::get_server_status(&server.host, server.port);
        let timeout_status = timeout(Duration::from_millis(100), server_status_future).await;
        let server_status =
            match timeout_status {
                Ok(server_status) => server_status,
                Err(_err) => { bail!("timed out!") }
            };
        match server_status {
            Ok(server_status) => {
                output::display_response(&server_status, &server);
                config.save()
            }
            Err(err) => return Err(err)
        }
    }
}
