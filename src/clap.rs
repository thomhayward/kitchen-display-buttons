use crate::config::ConfigOwned;
use clap::Parser;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(
        long = "config",
        default_value = "/etc/kitchen-display-buttons/config.yaml",
        env = "KITCHEN_DISPLAY_BUTTONS_CONFIG"
    )]
    config_path: PathBuf,
}

impl Arguments {
    pub fn configuration(&self) -> anyhow::Result<ConfigOwned> {
        let file = File::open(&self.config_path)?;
        let config = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}

pub fn parse() -> Arguments {
    Arguments::parse()
}
