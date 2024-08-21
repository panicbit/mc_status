use anyhow::Result;
use clap::Parser;
#[tokio::main]
async fn main() -> Result<()> {
    mc_status::Cli::parse().run().await
}
