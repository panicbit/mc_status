use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::APP_NAME;

#[derive(Serialize, Deserialize, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Server {
        Server { host, port }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub server_list: HashSet<Server>,
}

impl Config {
    const NAME: &'static str = "config";

    pub fn load() -> Result<Self> {
        let config = confy::load::<Self>(APP_NAME, Self::NAME)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        confy::store(APP_NAME, Self::NAME, self)?;

        Ok(())
    }
}
