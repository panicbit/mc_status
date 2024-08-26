use crate::{get_server_status, output, Config};
use anyhow::{Context, Result};
use futures::future;
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
        let mut result_futures = Vec::new();

        for server in config.server_list {
            let result_future = tokio::spawn(async move {
                let result = get_server_status(&server.host, server.port).await;

                (server, result)
            });

            result_futures.push(result_future);
        }

        for future_result in future::join_all(result_futures).await {
            let (server, server_status_result) = future_result.context("failed to run task")?;

            output::display_response_result(&server, server_status_result);
        }

        Ok(())
    }
}
