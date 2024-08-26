use crate::{config, get_server_status, output, Config};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;

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
        let server = config::Server::new(
            self.server.host.clone(),
            self.server.port,
            self.server.alias.clone(),
        );
        if self.overwrite_server(&config).is_break() {
            return Ok(());
        }

        config.server_list.insert(server.clone());
        let server_status = get_server_status(&server.host, server.port).await;

        output::display_response_result(&server, server_status);
        config.save()
    }

    fn overwrite_server(&self, config: &Config) -> ControlFlow<()> {
        let found_server = config
            .server_list
            .iter()
            .find(|server| self.server.alias == server.alias);

        if let Some(server) = found_server {
            let overwrite_confirmed = dialoguer::Confirm::new()
                .with_prompt(format!("Do you want to overwrite {}?", server.alias))
                .default(true)
                .interact()
                .unwrap();

            if overwrite_confirmed {
                return ControlFlow::Continue(());
            } else {
                return ControlFlow::Break(());
            }
        }

        ControlFlow::Continue(())
    }
}
