use crate::view::music::MusicSelectionState;

pub enum Views {
    MusicTable,
}

pub struct ViewContext {
    pub current: Views,
    pub music_selection_state: MusicSelectionState,
}

impl ViewContext {
    pub fn new() -> ViewContext {
        ViewContext {
            current: Views::MusicTable,
            music_selection_state: MusicSelectionState::default(),
        }
    }
}
