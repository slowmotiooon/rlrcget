use color_eyre::Result;
use std::path::PathBuf;

pub struct MusicContext {
    pub music_list: Vec<Music>,
    pub selected_idx: Option<usize>,
}

impl MusicContext {
    pub fn new() -> MusicContext {
        MusicContext {
            music_list: vec![],
            selected_idx: None,
        }
    }
}

pub struct Music {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub duration: i32,
    pub path: PathBuf,
}
