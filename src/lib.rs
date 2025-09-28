pub mod message;
pub mod model;
pub mod view;

use color_eyre::{Result, eyre::Ok};
use message::AppMsg;
use message::update;
use model::AppContext;
use ratatui::DefaultTerminal;

pub fn app(terminal: &mut DefaultTerminal, mut context: AppContext) -> Result<()> {
    update(&mut context, AppMsg::Init)?;
    while !context.exit {
        terminal.draw(|frame| view::draw(&context, frame))?;
        view::handle_events(&mut context)?;
    }
    Ok(())
}
