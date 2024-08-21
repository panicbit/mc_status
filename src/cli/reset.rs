use crate::Config;
use anyhow::Result;

#[derive(clap::Args, Clone)]
pub struct Cli {}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let mut config = Config::load()?;
        config.server_list.clear();
        config.save()?;
        Ok(())
    }
}