use color_eyre::Result;
use rlrcget::app;
use rlrcget::model::AppContext;

fn main() -> Result<()> {
    let context = AppContext::new()?;
    let mut terminal = ratatui::init();
    let result = app(&mut terminal, context);
    ratatui::restore();
    result
}
