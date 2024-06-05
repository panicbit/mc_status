
#[derive(clap::Parser)]
pub struct Cli {
    pub host: String,
    #[clap(default_value_t = 25565)]
    pub port: u16,
}
