pub mod args;
mod config;
mod music;
pub mod view;

use crate::model::{config::parse_config, view::ViewContext};
use color_eyre::Result;
use config::AppConfig;
use music::MusicContext;

pub struct AppContext {
    pub config: AppConfig,
    pub exit: bool,
    pub music: MusicContext,
    pub view: ViewContext,
}

impl AppContext {
    pub fn new() -> Result<AppContext> {
        Ok(AppContext {
            config: parse_config()?,
            exit: false,
            music: MusicContext::new(),
            view: ViewContext::new(),
        })
    }
}
