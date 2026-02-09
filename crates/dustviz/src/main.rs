// crates/dustviz/src/main.rs

use std::path::PathBuf;
use std::process;

use clap::Parser;

use dustviz::app::{run, AppConfig};
use dustviz::cli::{Cli, Command, OutputFormat};
use dustviz::graph::build_dir_graph;
use dustviz::input::{load_dir_program, resolve_input_path};
use dustviz::render::render_dot;
use dustviz::util::diagnostics::Diagnostic;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Parse { input } => cmd_parse(input),
        Command::Render {
            input,
            format,
            output,
        } => cmd_render(input, format, output),
    };

    if let Err(err) = result {
        report_error(err);
        process::exit(1);
    }
}

fn cmd_parse(input: PathBuf) -> Result<(), Diagnostic> {
    // Keep the same path resolution rules as the app layer.
    let _ = run(AppConfig { input })?;
    Ok(())
}

fn cmd_render(input: PathBuf, format: OutputFormat, output: Option<PathBuf>) -> Result<(), Diagnostic> {
    let input_path = resolve_input_path(&input)?;
    let program = load_dir_program(&input_path)?;

    let graph = build_dir_graph(&program);

    match format {
        OutputFormat::Dot => {
            let dot = render_dot(&graph);
            if let Some(out_path) = output {
                std::fs::write(&out_path, dot).map_err(|e| Diagnostic::Io {
                    path: out_path,
                    source: e,
                })?;
            } else {
                print!("{dot}");
            }
        }
        OutputFormat::Json | OutputFormat::Svg => {
            return Err(Diagnostic::message(
                "only --format dot is implemented in v0.1",
            ));
        }
    }

    Ok(())
}

fn report_error(err: Diagnostic) {
    eprintln!("error: {err}");
}