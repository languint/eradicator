use std::{fs, process::exit};

use clap::Parser;
use eradicator_core::strategy::StrategyParser;
use owo_colors::OwoColorize;

mod cli;
mod state;

fn main() {
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

            println!(
                "      {} strategy, mode={}, loadout={}, map={}",
                "Parsed".bright_green().bold(),
                strategy.header.mode.bold(),
                strategy
                    .header
                    .loadout
                    .to_vec()
                    .iter()
                    .filter(|t| t.is_some())
                    .map(|t| {
                        let tower = unsafe { t.unwrap_unchecked() };

                        format!("{}", tower.bold())
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
                strategy.header.map.bold()
            );
        }
    }
}
