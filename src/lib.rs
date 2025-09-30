pub mod message;
pub mod model;
pub mod view;

use color_eyre::Result;
use message::AppMsg;
use message::update;
use model::AppContext;
use ratatui::DefaultTerminal;

pub fn app(terminal: &mut DefaultTerminal, mut context: AppContext) -> Result<()> {
    context.tx.send(AppMsg::Init)?;
    while !context.exit {
        while let Ok(msg) = context.rx.try_recv() {
            update(&mut context, msg)?;
        }
        terminal.draw(|frame| view::draw(&mut context, frame))?;
        view::handle_events(&mut context)?;
    }
    Ok(())
}
