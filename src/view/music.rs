use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState},
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

        StatefulWidget::render(&table, area, buf, &mut state.music_table_state);
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
            .map(|m| {
                Row::new(vec![
                    Cell::from(m.title.as_str()),
                    Cell::from(m.artist.as_str()),
                    Cell::from(m.album.as_str()),
                    Cell::from(format!("{}s", m.duration)),
                ])
            })
            .collect::<Vec<Row>>();
        let widths = [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
        ];
        let header = Row::new(vec!["Title", "Artist", "Album", "Duration"])
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
