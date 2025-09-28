pub mod model;
pub mod view;

use color_eyre::{Result, eyre::Ok};
use model::AppContext;
use ratatui::DefaultTerminal;

pub fn app(terminal: &mut DefaultTerminal, mut context: AppContext) -> Result<()> {
    while !context.exit {
        terminal.draw(|frame| view::draw(&context, frame))?;
        view::handle_events(&mut context)?;
    }
    Ok(())
}
