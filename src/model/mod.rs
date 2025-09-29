pub mod args;
mod config;
pub mod lyrics;
pub mod music;
pub mod view;

use crate::model::{config::parse_config, lyrics::LyricsContext, view::ViewContext};
use color_eyre::Result;
use config::{AppConfig, ConfigSources};
use music::MusicContext;

/// The total model of the application, which stores all the states and configs.
pub struct AppContext {
    pub config: AppConfig,
    pub exit: bool,
    pub music: MusicContext,
    pub lyrics: LyricsContext,
    pub view: ViewContext,
}

impl AppContext {
    /// Obtain the initialized context.
    pub fn new() -> Result<AppContext> {
        Ok(AppContext {
            config: parse_config(ConfigSources::default())?,
            exit: false,
            music: MusicContext::new(),
            lyrics: LyricsContext::new(),
            view: ViewContext::new(),
        })
    }
}
