use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

use crate::model::AppContext;

pub struct MusicSelection<'a> {
    pub context: &'a AppContext,
}

impl<'a> Widget for &MusicSelection<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let _music_path = Text::from(self.context.config.path.music_path.to_str().unwrap());
        let _block = Block::new().style(Style::new().bg(Color::Red));

        let table = MusicTable {
            context: self.context,
        };

        table.render(area, buf);
    }
}

struct MusicTable<'a> {
    pub context: &'a AppContext,
}

impl<'a> Widget for &MusicTable<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self
            .context
            .music
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
            .style(Style::new().bold())
            .bottom_margin(1);
        let block = Block::new().title("Musics").borders(Borders::all());
        let table = Table::new(rows, widths).header(header).block(block);
        table.render(area, buf);
    }
}
