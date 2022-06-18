mod app;
mod ui;

use crate::app::run;
use anyhow::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}
