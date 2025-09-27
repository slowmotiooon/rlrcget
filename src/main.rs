use color_eyre::Result;
use rlrcget::model::config::parse_config;
use rlrcget::{AppContext, app};

fn main() -> Result<()> {
    let mut context = AppContext {
        config: parse_config()?,
        exit: false,
    };
    let mut terminal = ratatui::init();
    let result = app(&mut terminal, context);
    ratatui::restore();
    result
}
