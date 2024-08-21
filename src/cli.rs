use anyhow::Result;

pub mod add;
pub mod check;
pub mod delete;
pub mod list;
pub mod reset;

#[derive(clap::Parser)]
#[group(skip)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Some(command) => command.run().await,
            None => check::Cli {}.run().await,
        }
    }
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {
    Check(check::Cli),
    List(list::Cli),
    Delete(delete::Cli),
    Add(add::Cli),
    Reset(reset::Cli),
}

impl Command {
    pub async fn run(self) -> Result<()> {
        match self {
            Command::Check(cli) => cli.run().await,
            Command::List(cli) => cli.run().await,
            Command::Delete(cli) => cli.run().await,
            Command::Add(cli) => cli.run().await,
            Command::Reset(cli) => cli.run().await,
        }
    }
}
