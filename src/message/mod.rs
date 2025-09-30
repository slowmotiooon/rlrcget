pub mod lyrics;
pub mod music;

use lyrics::{LyricsMsg, lyrics_update};
use music::{MusicMsg, music_update};

use crate::model::{AppContext, view::Views};
use color_eyre::{Result, eyre::Ok};

pub enum AppMsg {
    Init,
    Music(MusicMsg),
    Lyrics(LyricsMsg),
    Exit,
    ChangeView(Views),
}

pub fn update(context: &mut AppContext, msg: AppMsg) -> Result<()> {
    match msg {
        AppMsg::Init => {
            update(context, AppMsg::Music(MusicMsg::LoadMusics))?;
            update(context, AppMsg::Music(MusicMsg::ChangeSelected(0)))?;
            update(context, AppMsg::Lyrics(LyricsMsg::LoadLyricsPreview))?;
            Ok(())
        }
        AppMsg::Music(m) => music_update(context, m),
        AppMsg::Lyrics(m) => lyrics_update(context, m),
        AppMsg::Exit => {
            context.exit = true;
            Ok(())
        }
        AppMsg::ChangeView(v) => {
            context.view.current = v;
            Ok(())
        }
    }
}
