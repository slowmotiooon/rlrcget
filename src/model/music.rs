use std::path::PathBuf;

pub struct MusicContext {
    pub music_list: Vec<Music>,
}

impl MusicContext {
    pub fn new() -> MusicContext {
        MusicContext { music_list: vec![] }
    }
}

pub struct Music {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub duration: u64,
    pub path: PathBuf,
}
