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

    /// Render a graph from a DIR program artifact.
    Render {
        /// Path to a DIR Program JSON file (e.g., program.dir.json)
        #[arg(long)]
        input: PathBuf,

        /// Optional constraint overlay JSON file (e.g., constraints.json)
        #[arg(long)]
        constraints: Option<PathBuf>,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Dot)]
        format: OutputFormat,

        /// Surface graph annotations (tooltips). Visible labels are unchanged.
        #[arg(long, default_value_t = false)]
        annotated: bool,

        /// Output path. If omitted, prints to stdout.
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