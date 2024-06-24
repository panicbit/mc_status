use anyhow::{Context, Result};

#[derive(clap::Args)]
pub struct Cli {
    pub host: String,
    #[clap(default_value_t = 25565)]
    pub port: u16,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let status_response = crate::get_server_status(&self.host, self.port)
            .context("failed to get server status")?;

        let players = status_response.players;

        println!("{} player(s) online:", players.online);

        for player in &players.sample {
            println!("{}", player.name);
        }

        if players.online > players.sample.len() as i32 {
            println!("...");
        }

        Ok(())
    }
}
