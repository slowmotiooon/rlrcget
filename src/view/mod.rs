pub mod music;

use crate::{
    AppContext,
    message::{AppMsg, music::MusicMsg, update},
    model::view::Views,
    view::music::MusicSelection,
};
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;

pub fn draw(context: &mut AppContext, frame: &mut Frame) {
    match context.view.current {
        Views::MusicTable => {
            let music_table = MusicSelection {
                music_context: &context.music,
            };
            frame.render_stateful_widget(
                &music_table,
                frame.area(),
                &mut context.view.music_selection_state,
            );
        }
    }
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
        KeyCode::Char('j') | KeyCode::Down => update(context, AppMsg::Music(MusicMsg::SelectNext)),
        KeyCode::Char('k') | KeyCode::Up => {
            update(context, AppMsg::Music(MusicMsg::SelectPrevious))
        }
        _ => Ok(()),
    }
}
