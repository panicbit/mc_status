use crate::Config;
use anyhow::Result;

#[derive(clap::Args, Clone)]
pub struct Cli {}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let config = Config::load()?;
        for server in &config.server_list {
            println!("{:?}", server);
        }
        Ok(())
    }
}
