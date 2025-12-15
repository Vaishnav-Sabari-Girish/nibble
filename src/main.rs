#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod error;
mod style;
mod tui;
mod widgets;

use clap::{Parser, Subcommand};
use widgets::{block, gauge, table, input, confirm};

#[derive(Parser)]
#[command(name = "nibble")]
#[command(about = "A tool for glamorous shell scripts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a block with borders and title
    Block(block::BlockArgs),
    /// Render a gauge/progress bar
    Gauge(gauge::GaugeArgs),
    /// Render a table
    Table(table::TableArgs),
    /// Render User Input
    Input(input::InputArgs),
    /// Render confirmation buttons
    Confirm(confirm::ConfirmArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Block(args) => block::run(args)?,
        Commands::Gauge(args) => gauge::run(args)?,
        Commands::Table(args) => table::run(args)?,
        Commands::Input(args) => input::run(args)?,
        Commands::Confirm(args) => input::run(args)?,
    }

    Ok(())
}
