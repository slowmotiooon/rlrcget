pub mod args;
mod config;
pub mod lyric;
pub mod music;
pub mod view;

use crate::model::{config::parse_config, view::ViewContext};
use color_eyre::Result;
use config::{AppConfig, ConfigSources};
use music::MusicContext;

/// The total model of the application, which stores all the states and configs.
pub struct AppContext {
    pub config: AppConfig,
    pub exit: bool,
    pub music: MusicContext,
    pub view: ViewContext,
}

impl AppContext {
    /// Obtain the initialized context.
    pub fn new() -> Result<AppContext> {
        Ok(AppContext {
            config: parse_config(ConfigSources::default())?,
            exit: false,
            music: MusicContext::new(),
            view: ViewContext::new(),
        })
    }
}
