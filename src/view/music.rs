use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::model::music::MusicContext;

pub struct MusicSelection<'a> {
    pub music_context: &'a MusicContext,
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
        let lyric = LyricBlock;
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
        let rows = self
            .music_context
            .music_list
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
        let block = Block::new().title("Musics").borders(Borders::all());
        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .row_highlight_style(Style::default().reversed());
        StatefulWidget::render(table, area, buf, state);
    }
}

pub struct LyricBlock;

impl Widget for &LyricBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().title("Lyric").borders(Borders::all());
        Widget::render(block, area, buf);
    }
}
