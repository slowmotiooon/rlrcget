use crate::{model::AppContext, model::lyrics::Lyrics};
use color_eyre::Result;

pub enum LyricsMsg {
    LoadLyricsPreview,
}

pub fn lyrics_update(context: &mut AppContext, msg: LyricsMsg) -> Result<()> {
    match msg {
        LyricsMsg::LoadLyricsPreview => load_lyrics_preview(context),
    }
}

fn load_lyrics_preview(context: &mut AppContext) -> Result<()> {
    if let Some(selected) = context
        .view
        .music_selection_state
        .music_table_state
        .selected()
    {
        if let Some(music) = context.music.music_list().get(selected) {
            let mut lyrics_path = music.path.clone();
            lyrics_path.set_extension("lrc");
            if lyrics_path.exists() {
                let content = std::fs::read_to_string(lyrics_path)?;
                let lyrics = Lyrics::from_str(content);
                context.lyrics.current_lyrics = Some(lyrics);
            } else {
                context.lyrics.current_lyrics = None;
            }
        }
    }
    Ok(())
}
