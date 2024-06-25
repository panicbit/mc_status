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
    pub fn run(self) -> Result<()> {
        match self.command {
            Some(command) => command.run(),
            None => check::Cli {}.run(),
        }
    }
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {
    Check(check::Cli),
    List(list::Cli),
    Delete(delete::Cli),
    Add(add::Cli),
    Reset(reset::Cli)
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Command::Check(cli) => cli.run(),
            Command::List(cli) => cli.run(),
            Command::Delete(cli) => cli.run(),
            Command::Add(cli) => cli.run(),
            Command::Reset(cli) => cli.run(),
        }
    }
}
