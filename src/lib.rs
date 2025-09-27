pub mod model;

use model::config::AppConfig;
use std::error::Error;

pub fn app(config: AppConfig) -> Result<(), Box<dyn Error>> {
    Ok(())
}
