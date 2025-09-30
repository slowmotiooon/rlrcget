use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, StatefulWidget, Widget},
};

use crate::model::{lyrics::LyricsContext, music::MusicContext};

pub struct LyricsEdition<'a> {
    pub music_context: &'a mut MusicContext,
    pub lyrics_context: &'a mut LyricsContext,
}

pub struct LyricsEditionState;

impl<'a> StatefulWidget for &LyricsEdition<'a> {
    type State = LyricsEditionState;
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let block = Block::default()
            .title("Lyric Edition")
            .borders(ratatui::widgets::Borders::ALL);
        Widget::render(block, area, buf);
    }
}

impl LyricsEditionState {
    pub fn default() -> LyricsEditionState {
        LyricsEditionState
    }
}
