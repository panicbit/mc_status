use crate::{config, get_server_status, output, Config};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io;
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
        if self
            .overwrite_server(&config, &self.server.alias)
            .is_break()
        {
            return Ok(());
        }

        config.server_list.insert(server.clone());
        let server_status = get_server_status(&server.host, server.port)
            .await
            .context("failed to get server status")?;

        output::display_response(&server_status, &server);
        config.save()
    }
    fn overwrite_server(&self, config: &Config, input_alias: &String) -> ControlFlow<()> {
        let found_server = config
            .server_list
            .iter()
            .find(|server| input_alias == &server.alias);
        if let Some(server) = found_server {
            loop {
                println!("Do you want to overwrite {}? (Y/n)", server.alias);
                let mut choice = String::new();
                choice.clear();
                io::stdin().read_line(&mut choice).unwrap();
                let choice = choice.trim();
                if choice == "Y" || choice.is_empty() {
                    return ControlFlow::Continue(());
                }
                if choice == "n" {
                    return ControlFlow::Break(());
                }
                println!("Invalid input, try again");
            }
        }
        ControlFlow::Continue(())
    }
}
