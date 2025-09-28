pub mod music;

use crate::{
    AppContext,
    message::{AppMsg, update},
    model::view::Views,
    view::music::MusicSelection,
};
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;

pub fn draw(context: &AppContext, frame: &mut Frame) {
    let music_table = MusicSelection { context };
    frame.render_widget(
        match context.view.current {
            Views::MusicTable => &music_table,
        },
        frame.area(),
    );
}

pub fn handle_events(context: &mut AppContext) -> Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(context, key_event)
        }
        _ => Ok(()),
    }
}

fn handle_key_event(context: &mut AppContext, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        KeyCode::Char('q') => update(context, AppMsg::Exit),
        _ => Ok(()),
    }
}
