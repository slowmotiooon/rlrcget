use std::path::PathBuf;

use crate::model::AppContext;
use crate::model::music::Music;
use color_eyre::{Report, Result};
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use walkdir::WalkDir;

pub enum MusicMsg {
    MusicUpdate,
    ChangeSelected(usize),
}

pub fn music_update(context: &mut AppContext, msg: MusicMsg) -> Result<()> {
    match msg {
        MusicMsg::MusicUpdate => update_music_list(context),
        MusicMsg::ChangeSelected(idx) => Ok(select_music(context, idx)),
    }
}

fn update_music_list(context: &mut AppContext) -> Result<()> {
    let mut music_list = vec![];
    for entry in WalkDir::new(context.config.path.music_path.clone()) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Ok(tagged_file) = Probe::open(entry.path()).expect("Bad path provided").read() {
                let tag = match tagged_file.primary_tag() {
                    Some(primary_tag) => primary_tag,
                    None => tagged_file.first_tag().expect("No tags found."),
                };

                let title = tag
                    .title()
                    .as_deref()
                    .expect("Title parse error.")
                    .to_string();
                let album = tag
                    .album()
                    .as_deref()
                    .expect("Album parse error.")
                    .to_string();
                let artist = tag
                    .artist()
                    .as_deref()
                    .expect("Artist parse error.")
                    .to_string();
                let duration = tagged_file.properties().duration().as_secs();
                let path = PathBuf::from(entry.path());
                let music = Music {
                    title,
                    album,
                    artist,
                    duration,
                    path,
                };
                music_list.push(music);
            }
        }
    }
    context.music.music_list = music_list;
    Ok(())
}

fn select_music(context: &mut AppContext, idx: usize) {
    context.music.selected_idx = if context.music.music_list.is_empty() {
        None
    } else {
        Some(idx)
    };
}
