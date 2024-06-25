use anyhow::{Result};
use crate::{Config};


#[derive(clap::Args, Clone)]
pub struct Cli {}

impl Cli {
    pub fn run(self) -> Result<()> {
        let config = Config::load()?;
        for server in &config.server_list {
            println!("{:?}", server);
        }
        Ok(())
    }
}