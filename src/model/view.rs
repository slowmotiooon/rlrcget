use crate::view::edition::LyricsEditionState;
use crate::view::selection::MusicSelectionState;

pub enum Views {
    MusicSelection,
    LyricsEdition,
}

pub struct ViewContext {
    pub current: Views,
    pub music_selection_state: MusicSelectionState,
    pub lyrics_edition_state: LyricsEditionState,
}

impl ViewContext {
    pub fn new() -> ViewContext {
        ViewContext {
            current: Views::MusicSelection,
            music_selection_state: MusicSelectionState::default(),
            lyrics_edition_state: LyricsEditionState::default(),
        }
    }
}
