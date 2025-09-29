use std::path::PathBuf;

use crate::model::AppContext;
use crate::model::music::Music;
use color_eyre::Result;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use walkdir::WalkDir;

pub enum MusicMsg {
    MusicUpdate,
    ChangeSelected(usize),
    SelectPrevious,
    SelectNext,
}

pub fn music_update(context: &mut AppContext, msg: MusicMsg) -> Result<()> {
    match msg {
        MusicMsg::MusicUpdate => update_music_list(context),
        MusicMsg::ChangeSelected(idx) => Ok(select_music(context, idx)),
        MusicMsg::SelectNext => Ok(select_next(context)),
        MusicMsg::SelectPrevious => Ok(select_previous(context)),
    }
}

fn update_music_list(context: &mut AppContext) -> Result<()> {
    let mut music_list = vec![];
    let music_path = match &context.config.path.music_path {
        Some(p) => p,
        None => {
            return Err(color_eyre::Report::msg(
                "No music folder. Please add the music folder in the config file.",
            ));
        }
    };
    for entry in WalkDir::new(music_path).follow_links(true).max_depth(25) {
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
    context.view.music_selection_state.music_table_state.select(
        if context.music.music_list.is_empty() {
            None
        } else {
            Some(idx)
        },
    );
}

fn select_next(context: &mut AppContext) {
    if let Some(idx) = context
        .view
        .music_selection_state
        .music_table_state
        .selected()
    {
        if context.music.music_list.len() > idx + 1 {
            select_music(context, idx + 1);
        }
    }
}

fn select_previous(context: &mut AppContext) {
    if let Some(idx) = context
        .view
        .music_selection_state
        .music_table_state
        .selected()
    {
        if idx > 0 {
            select_music(context, idx - 1);
        }
    }
}
