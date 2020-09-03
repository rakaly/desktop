use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Serialize)]
pub struct UserInputConfig {
    pub username: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct UploaderConfig {
    pub username: String,
    pub api_key: String,
    pub watch_directory: Option<PathBuf>,

    #[serde(default = "default_api_url")]
    pub api_url: String,

    #[serde(default = "default_log_level")]
    pub log_level: log::LevelFilter,
}

fn default_api_url() -> String {
    String::from("https://rakaly.com/api/upload")
}

fn default_log_level() -> log::LevelFilter {
    log::LevelFilter::Info
}

pub fn read_config<P: AsRef<Path>>(location: P) -> anyhow::Result<UploaderConfig> {
    let path = location.as_ref();
    let config_data =
        std::fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let config = parse_config(&config_data)
        .with_context(|| format!("Malformatted config file: {}", path.display()))?;
    Ok(config)
}

pub fn parse_config(data: &[u8]) -> anyhow::Result<UploaderConfig> {
    toml::de::from_slice(data).context("unable to deserialize toml config")
}

pub fn write_minimal_config<P: AsRef<Path>>(
    input: &UserInputConfig,
    destination: P,
) -> anyhow::Result<UploaderConfig> {
    let path = destination.as_ref();
    let config_data =
        toml::ser::to_vec(&input).context("unable to serialize user input to a config")?;

    std::fs::write(path, &config_data)
        .with_context(|| format!("Unable to write config file: {}", path.display()))?;

    parse_config(&config_data)
}

pub fn get_user_input() -> UserInputConfig {
    let _ = writeln!(io::stdout(), "Welcome to Rakaly.");
    let _ = writeln!(
        io::stdout(),
        "A config file was not detected, so we'll create one"
    );
    let _ = write!(io::stdout(), "Steam username: ");
    let _ = io::stdout().flush();
    let username: String = text_io::read!("{}\n");
    let _ = write!(io::stdout(), "{}'s API key: ", username);
    let _ = io::stdout().flush();
    let api_key: String = text_io::read!("{}\n");
    UserInputConfig { username, api_key }
}
