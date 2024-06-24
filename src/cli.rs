use anyhow::Result;

pub mod main;

#[derive(clap::Parser)]
#[clap(args_conflicts_with_subcommands = true)]
#[group(skip)]
pub struct Cli {
    #[clap(flatten)]
    main: Option<main::Cli>,
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        if let Some(main) = self.main {
            return main.run();
        }

        self.command.unwrap().run()
    }
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {}
    }
}
