// crates/dustviz/src/cli.rs

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "dustviz",
    version,
    about = "IR + constraint graph visualization",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Parse a DIR program artifact (JSON) to verify it is well-formed.
    Parse {
        /// Path to a DIR Program JSON file (e.g., program.dir.json)
        #[arg(long)]
        input: PathBuf,
    },

    /// Render a graph (v0.1: loads input only; rendering lands next).
    Render {
        /// Path to a DIR Program JSON file (e.g., program.dir.json)
        #[arg(long)]
        input: PathBuf,

        /// Output format (v0.1 accepts the flag but does not render yet)
        #[arg(long, value_enum, default_value_t = OutputFormat::Dot)]
        format: OutputFormat,

        /// Output path (v0.1 accepts the flag but does not write yet)
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Dot,
    Json,
    Svg,
}