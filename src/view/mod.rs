pub mod edition;
pub mod selection;

use crate::{
    AppContext,
    message::{AppMsg, lyrics::LyricsMsg, music::MusicMsg, update},
    model::view::Views,
    view::{edition::LyricsEdition, selection::MusicSelection},
};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;

pub fn draw(context: &mut AppContext, frame: &mut Frame) {
    match context.view.current {
        Views::MusicSelection => {
            let music_table = MusicSelection {
                music_context: &context.music,
                lyrics_context: &context.lyrics,
            };
            frame.render_stateful_widget(
                &music_table,
                frame.area(),
                &mut context.view.music_selection_state,
            );
        }
        Views::LyricsEdition => {
            let lyric_edition = LyricsEdition {
                music_context: &mut context.music,
                lyrics_context: &mut context.lyrics,
            };
            frame.render_stateful_widget(
                &lyric_edition,
                frame.area(),
                &mut context.view.lyrics_edition_state,
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
    match context.view.current {
        Views::MusicSelection => handle_music_table_key_event(context, key_event),
        Views::LyricsEdition => handle_lyrics_edition_key_event(context, key_event),
    }
}

fn handle_music_table_key_event(context: &mut AppContext, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        KeyCode::Char('q') => update(context, AppMsg::Exit),
        KeyCode::Char('j') | KeyCode::Down => {
            update(context, AppMsg::Music(MusicMsg::SelectNext))?;
            update(context, AppMsg::Lyrics(LyricsMsg::LoadLyricsPreview))?;
            Ok(())
        }
        KeyCode::Char('k') | KeyCode::Up => {
            update(context, AppMsg::Music(MusicMsg::SelectPrevious))?;
            update(context, AppMsg::Lyrics(LyricsMsg::LoadLyricsPreview))?;
            Ok(())
        }
        KeyCode::Char('r') | KeyCode::F(5) => {
            update(context, AppMsg::Music(MusicMsg::UpdateLibrary))?;
            Ok(())
        }
        KeyCode::Enter | KeyCode::Char('l') => {
            update(context, AppMsg::ChangeView(Views::LyricsEdition))?;
            Ok(())
        }
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            update(context, AppMsg::Exit)
        }
        _ => Ok(()),
    }
}

fn handle_lyrics_edition_key_event(context: &mut AppContext, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('h') => {
            update(context, AppMsg::ChangeView(Views::MusicSelection))?;
            Ok(())
        }
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            update(context, AppMsg::Exit)
        }
        _ => Ok(()),
    }
}
