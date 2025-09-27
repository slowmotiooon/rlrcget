use super::args::Args;
use clap::Parser;
use color_eyre::Result;
use config::Config;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub path: PathConfig,
}

#[derive(Deserialize, Debug)]
pub struct PathConfig {
    pub music_path: PathBuf,
    pub lrc_path: Option<PathBuf>,
}

pub fn parse_config() -> Result<AppConfig> {
    // first: from arguments
    let mut config_builder = Config::builder();
    if let Some(p) = Args::parse().config {
        if p.exists() && p.is_file() {
            config_builder = config_builder.add_source(config::File::from(p));
        }
    }
    // second: from XDG_CONFIG_HOME
    if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
        let mut config_path = PathBuf::from_str(&config_home)?;
        config_path.push(env!("CARGO_PKG_NAME"));
        config_path.push("config.toml");
        config_builder = config_builder.add_source(config::File::from(config_path));
    }
    // third: from ~/.config
    let mut user_config_path = PathBuf::from_str(&env::var("HOME")?)?;
    user_config_path.push(env!("CARGO_PKG_NAME"));
    user_config_path.push("config.toml");
    if user_config_path.exists() {
        config_builder = config_builder.add_source(config::File::from(user_config_path));
    }
    // fourth: from project
    let proj_config_path = PathBuf::from_str("config/config.toml")?;
    if proj_config_path.exists() {
        config_builder = config_builder.add_source(config::File::from(proj_config_path));
    }

    match config_builder.build()?.try_deserialize::<AppConfig>() {
        Ok(config) => Ok(config),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_path_test() {
        let config = PathBuf::from_str("~/.config");
        println!("{:?}", config);
    }
}
