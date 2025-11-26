use std::path::PathBuf;

use clap::{Parser, Subcommand};
use owo_colors::{AnsiColors, DynColors, OwoColorize};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run {
        #[arg(short, long)]
        strat: PathBuf,
    },
}
