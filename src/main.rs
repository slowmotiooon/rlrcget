use color_eyre::Result;
use rlrcget::app;
use rlrcget::model::AppContext;

/// Entrance of the application.
fn main() -> Result<()> {
    // Initialize color_eyre for error handling.
    color_eyre::install()?;
    // Initialize Application Context
    let context = AppContext::new()?;
    // Initialize ratatui terminal.
    let mut terminal = ratatui::init();
    // Run the app
    let result = app(&mut terminal, context);
    ratatui::restore();
    result
}
