use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

pub struct MusicContext {
    pub music_list: Arc<ArcSwap<Vec<Music>>>,
}

impl MusicContext {
    pub fn new() -> MusicContext {
        MusicContext {
            music_list: Arc::new(ArcSwap::from_pointee(vec![])),
        }
    }

    pub fn music_list(&self) -> Arc<Vec<Music>> {
        self.music_list.load_full()
    }

    pub fn set_music_list(&self, new_list: Vec<Music>) {
        self.music_list.store(Arc::new(new_list));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Music {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub duration: u64,
    pub path: PathBuf,
}
