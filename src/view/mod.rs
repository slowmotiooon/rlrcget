use crate::AppContext;
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct AppView<'a> {
    pub context: &'a AppContext,
}

impl<'a> Widget for &AppView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("ratatui app test".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);
        let text = Text::from(vec![Line::from(vec![
            "music path: ".into(),
            self.context
                .config
                .path
                .music_path
                .to_str()
                .unwrap()
                .yellow(),
        ])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

pub fn draw(context: &AppContext, frame: &mut Frame) {
    let view = AppView { context };
    frame.render_widget(&view, frame.area());
}

pub fn handle_events(context: &mut AppContext) -> Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(context, key_event)
        }
        _ => {}
    }
    Ok(())
}

fn handle_key_event(context: &mut AppContext, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => context.exit = true,
        _ => {}
    }
}
