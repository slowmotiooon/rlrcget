use rlrcget::app;
use rlrcget::model::config::parse_config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app_config = parse_config()?;
    app(app_config)
}
