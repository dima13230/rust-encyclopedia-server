use serde::Deserialize;
use config::{Config, ConfigError, File};

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Settings> = Lazy::new(|| { Settings::new().expect("Config can't be loaded") } );

#[derive(Debug, Deserialize,Clone)]
pub struct Server {
    pub path: String,
    pub database_path: String,
    pub articles_path: String
}

#[derive(Debug, Deserialize,Clone)]
pub struct Commands {
    pub articles: String,
    pub articles_list: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub commands: Commands
}

const CONFIG_FILE_PATH: &str = "./config/default.toml";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::new();

        conf.merge(File::with_name(CONFIG_FILE_PATH))?;

        conf.try_into()
    }
}