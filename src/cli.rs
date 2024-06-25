use anyhow::Result;

pub mod check;
pub mod list;
pub mod delete;
pub mod add;


#[derive(clap::Parser)]
#[group(skip)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Command>
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command
        {
            Some(command) => {command.run()}
            None => {check::Cli{}.run()}
        }
    }
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {
    Check(check::Cli),
    List(list::Cli),
    Delete(delete::Cli),
    Add(add::Cli)
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Command::Check(cli) => {
                cli.run()
            }
            Command::List(cli) => {
                cli.run()
            }
            Command::Delete(cli) => {
                cli.run()
            }
            Command::Add(cli) => {
                cli.run()
            }
        }
    }
}
