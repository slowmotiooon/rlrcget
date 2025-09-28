pub mod music;

use music::MusicMsg;

use crate::{message::music::music_update, model::AppContext};
use color_eyre::{Result, eyre::Ok};

pub enum AppMsg {
    Init,
    Music(MusicMsg),
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
        AppMsg::Exit => {
            context.exit = true;
            Ok(())
        }
    }
}
