use clap::Parser;

/// A terminal-based typing test game
#[derive(Parser)]
#[clap(name = "tappie")]
#[clap(version, about)]
pub struct Cli;
