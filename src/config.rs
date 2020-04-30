// config and cli opts

// namespacing
use crate::Result;
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

// config struct (duh)
#[derive(Deserialize)]
pub struct Config {
    pub development_token: String,
    pub production_token: String,
    pub command_prefix: String,
    pub moderation_channel_id: u64,
}

// using trait cause idk???
impl std::convert::TryFrom<PathBuf> for Config {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: PathBuf) -> Result<Self> {
        let raw = std::fs::read_to_string(&path)?;
        let cfg = toml::from_str(&raw)?;
        Ok(cfg)
    }
}

// cli args
#[derive(StructOpt)]
#[structopt(name = "cistern 2.0", about = "hot trash discord bot")]
pub struct Options {
    /// launch the bot in production mode
    #[structopt(short = "p", long = "production")]
    pub use_production_token: bool,

    /// specify a nonstandard config path
    #[structopt(short = "c", long = "config")]
    pub configuration_path: Option<PathBuf>,
}
