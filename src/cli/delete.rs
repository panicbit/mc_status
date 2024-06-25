use crate::Config;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, clap::Args)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(clap::Args, Clone)]
pub struct Cli {
    #[clap(flatten)]
    pub server: Server,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let mut config = Config::load()?;
        config
            .server_list
            .retain(|server| server.host != self.server.host || server.port != self.server.port);
        config.save()?;
        Ok(())
    }
}
