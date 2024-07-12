use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Perstistent config information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn new() -> Config {
        Config {}
    }

    pub fn load() -> Config {
        todo!()
    }

    pub fn save() -> Result<()> {
        Err(anyhow!("Not implemented"))
    }
}
