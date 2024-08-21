use crate::Config;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, clap::Args)]
pub struct Server {
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
        config
            .server_list
            .retain(|server| server.alias != self.server.alias);
        config.save()?;
        Ok(())
    }
}
