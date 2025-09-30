use crate::model::lyrics::LyricsState;
use crate::model::{lyrics::LyricsContext, music::MusicContext};
use ratatui::text::Text;
use ratatui::widgets::Wrap;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};

pub struct MusicSelection<'a> {
    pub music_context: &'a MusicContext,
    pub lyrics_context: &'a LyricsContext,
}

pub struct MusicSelectionState {
    pub music_table_state: TableState,
}

impl MusicSelectionState {
    pub fn default() -> MusicSelectionState {
        MusicSelectionState {
            music_table_state: TableState::default(),
        }
    }
}

impl<'a> StatefulWidget for &MusicSelection<'a> {
    type State = MusicSelectionState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let table = MusicTable {
            music_context: self.music_context,
        };
        let lyric = LyricBlock {
            context: self.lyrics_context,
        };
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(area);
        Widget::render(&lyric, layout[1], buf);

        StatefulWidget::render(&table, layout[0], buf, &mut state.music_table_state);
    }
}

struct MusicTable<'a> {
    pub music_context: &'a MusicContext,
}

impl<'a> StatefulWidget for &MusicTable<'a> {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::new().title("Musics").borders(Borders::all());
        let music_list = self.music_context.music_list();
        if music_list.is_empty() {
            let no_music_str = "No music.
            If this is your first time opening this app, please refresh it with <r> or <F5>.";
            let no_music_paragraph = Paragraph::new(Text::from(no_music_str))
                .block(block)
                .wrap(Wrap { trim: true });
            Widget::render(no_music_paragraph, area, buf);
            return;
        }
        let rows = music_list
            .iter()
            .enumerate()
            .map(|(idx, m)| {
                Row::new(vec![
                    Cell::from(format!("{}", idx + 1)),
                    Cell::from(m.title.as_str()),
                    Cell::from(m.artist.as_str()),
                    Cell::from(m.album.as_str()),
                    Cell::from(format!("{}s", m.duration)),
                ])
            })
            .collect::<Vec<Row>>();
        let widths = [
            Constraint::Percentage(5),
            Constraint::Percentage(35),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
        ];
        let header = Row::new(vec!["No.", "Title", "Artist", "Album", "Duration"])
            .style(Style::new().bold().blue())
            .bottom_margin(1);
        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .row_highlight_style(Style::default().reversed());
        StatefulWidget::render(table, area, buf, state);
    }
}

pub struct LyricBlock<'a> {
    context: &'a LyricsContext,
}

impl<'a> Widget for &LyricBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new().title("Lyric").borders(Borders::all());
        if let Some(lyrics) = &self.context.current_lyrics {
            let state_color = match lyrics.state() {
                LyricsState::Synced => Style::default().green(),
                LyricsState::Unsynced => Style::default().yellow(),
                LyricsState::Empty => Style::default(),
            };
            block = block.border_style(state_color).title(match lyrics.state() {
                LyricsState::Synced => "Synced",
                LyricsState::Unsynced => "Unsynced",
                LyricsState::Empty => "Empty",
            });
            let lyrics_list = Table::new(
                lyrics.lines.iter().map(|line| -> Row {
                    let timestamp = if let Some(ts) = line.timestamp {
                        format!(
                            "[{:02}:{:02}.{:03}]",
                            ts.as_secs() / 60,
                            ts.as_secs() % 60,
                            ts.subsec_millis()
                        )
                    } else {
                        "[--:--.---]".to_string()
                    };
                    let ts_cell = Cell::from(timestamp).style(Style::default().blue().italic());
                    Row::new(vec![ts_cell, Cell::from(line.text.as_str())])
                }),
                [Constraint::Length(11), Constraint::Min(10)],
            )
            .block(block);
            Widget::render(lyrics_list, area, buf);
        } else {
            let empty_message = Paragraph::new("No lyrics available.").block(block).style(
                Style::default()
                    .fg(ratatui::style::Color::DarkGray)
                    .italic(),
            );
            Widget::render(empty_message, area, buf);
        }
    }
}
