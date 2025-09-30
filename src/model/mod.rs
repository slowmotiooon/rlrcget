pub mod args;
mod config;
pub mod lyrics;
pub mod music;
pub mod view;

use crate::{
    message::AppMsg,
    model::{config::parse_config, lyrics::LyricsContext, view::ViewContext},
};
use color_eyre::Result;
use config::{AppConfig, ConfigSources};
use crossbeam_channel::{Receiver, Sender};
use music::MusicContext;

/// The total model of the application, which stores all the states and configs.
pub struct AppContext {
    pub config: AppConfig,
    pub exit: bool,
    pub music: MusicContext,
    pub lyrics: LyricsContext,
    pub view: ViewContext,
    pub tx: Sender<AppMsg>,
    pub rx: Receiver<AppMsg>,
}

impl AppContext {
    /// Obtain the initialized context.
    pub fn new() -> Result<AppContext> {
        let (tx, rx) = crossbeam_channel::unbounded();
        Ok(AppContext {
            config: parse_config(ConfigSources::default())?,
            exit: false,
            music: MusicContext::new(),
            lyrics: LyricsContext::new(),
            view: ViewContext::new(),
            tx,
            rx,
        })
    }
}
