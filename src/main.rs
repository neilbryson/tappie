mod app;
mod cli;
mod ui;

use crate::app::run;
use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _args: Cli = Cli::parse();
    run()?;
    Ok(())
}
