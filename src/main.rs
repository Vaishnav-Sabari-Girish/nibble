#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod error;
mod tui;
mod widgets;

use clap::{Parser, Subcommand};
use widgets::{block, gauge, table};

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Block(args) => block::run(args)?,
        Commands::Gauge(args) => gauge::run(args)?,
        Commands::Table(args) => table::run(args)?,
    }

    Ok(())
}
