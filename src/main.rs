use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    mc_status::Cli::parse().run()
}
