use super::args::Args;
use clap::Parser;
use color_eyre::Result;
use config::{Config, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

static DEFAULT_CONFIG: &str = include_str!("../../config/config.toml");

/// The application configuration.
#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub path: PathConfig,
}

/// The path part of the configuration.
#[derive(Deserialize, Debug)]
pub struct PathConfig {
    pub music_path: Option<PathBuf>,
    pub lrc_path: Option<PathBuf>,
    pub database_path: Option<PathBuf>,
}

pub struct ConfigSources {
    pub cli: Option<PathBuf>,
    pub xdg_config_home: Option<PathBuf>,
    pub home: Option<PathBuf>,
}

impl ConfigSources {
    /// Collect config files from cli, XDG_CONFIG_HOME and HOME.
    pub fn default() -> ConfigSources {
        let cli = Args::parse().config;
        let xdg_config_home = if let Ok(h) = env::var("XDG_CONFIG_HOME") {
            PathBuf::from_str(&h).map_or(None, |p| {
                Some(p.join(env!("CARGO_PKG_NAME")).join("config.toml"))
            })
        } else {
            None
        };
        let home = if let Some(h) = env::home_dir() {
            Some(
                h.join(".config")
                    .join(env!("CARGO_PKG_NAME"))
                    .join("config.toml"),
            )
        } else {
            None
        };
        ConfigSources {
            cli,
            xdg_config_home,
            home,
        }
    }
}

/// Get config from arguments or/and files.
pub fn parse_config(sources: ConfigSources) -> Result<AppConfig> {
    let mut config_builder =
        Config::builder().add_source(config::File::from_str(DEFAULT_CONFIG, FileFormat::Toml));

    if let Some(p) = sources.home {
        if p.exists() {
            config_builder = config_builder.add_source(config::File::from(p));
        }
    }
    if let Some(p) = sources.xdg_config_home {
        if p.exists() {
            config_builder = config_builder.add_source(config::File::from(p));
        }
    }
    if let Some(p) = sources.cli {
        if p.exists() {
            config_builder = config_builder.add_source(config::File::from(p));
        }
    }

    match config_builder.build()?.try_deserialize::<AppConfig>() {
        Ok(config) => Ok(config),
        Err(e) => Err(e.into()),
    }
}
