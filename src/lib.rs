//! Hi! :), this is my implementation of the tp1
use std::error::Error;

use config::Config;
pub mod config;

/// Function which runs the api
pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
