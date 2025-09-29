pub mod lyrics;
pub mod music;

use lyrics::{LyricsMsg, lyrics_update};
use music::{MusicMsg, music_update};

use crate::model::AppContext;
use color_eyre::{Result, eyre::Ok};

pub enum AppMsg {
    Init,
    Music(MusicMsg),
    Lyrics(LyricsMsg),
    Exit,
}

pub fn update(context: &mut AppContext, msg: AppMsg) -> Result<()> {
    match msg {
        AppMsg::Init => {
            update(context, AppMsg::Music(MusicMsg::MusicUpdate))?;
            update(context, AppMsg::Music(MusicMsg::ChangeSelected(0)))?;
            Ok(())
        }
        AppMsg::Music(m) => music_update(context, m),
        AppMsg::Lyrics(m) => lyrics_update(context, m),
        AppMsg::Exit => {
            context.exit = true;
            Ok(())
        }
    }
}
