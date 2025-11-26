use std::{fs, process::exit};

use clap::Parser;
use eradicator_core::strategy::StrategyParser;
use owo_colors::OwoColorize;

mod cli;
mod state;

fn main() -> Result<(), String> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Run { strat } => {
            println!(
                "    {} `{}`",
                "Loading".bright_green().bold(),
                strat.display()
            );

            let file_contents: String = match fs::read_to_string(&strat) {
                Ok(contents) => contents,
                Err(e) => {
                    println!(
                        "{}: failed to read strategy file `{e}`",
                        "error".bright_red().bold()
                    );
                    exit(1)
                }
            };

            println!("      {} strategy file", "Loaded".bright_green().bold());

            let mut parser = StrategyParser::new(&file_contents);

            println!(
                "    {} `{}`",
                "Parsing".bright_green().bold(),
                strat.display()
            );

            let strategy = match parser.parse() {
                Ok(strategy) => strategy,
                Err(e) => {
                    println!(
                        "{}: failed to parse strategy file `{e}`",
                        "error".bright_red().bold()
                    );
                    exit(1)
                }
            };

            println!("      {} strategy file", "Parsed".bright_green().bold());
        }
    }

    Ok(())
}
