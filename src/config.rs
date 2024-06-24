use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::APP_NAME;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {}

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
