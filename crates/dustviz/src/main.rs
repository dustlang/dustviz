// crates/dustviz/src/main.rs

use std::path::PathBuf;
use std::process;

use clap::Parser;

use dustviz::app::{run, AppConfig};
use dustviz::cli::{Cli, Command, OutputFormat};
use dustviz::graph::build_dir_graph;
use dustviz::input::{load_dir_program, resolve_input_path};
use dustviz::render::{render_dot, render_dot_annotated, render_json};
use dustviz::util::diagnostics::Diagnostic;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Parse { input } => cmd_parse(input),
        Command::Render {
            input,
            format,
            annotated,
            output,
        } => cmd_render(input, format, annotated, output),
    };

    if let Err(err) = result {
        report_error(err);
        process::exit(1);
    }
}

fn cmd_parse(input: PathBuf) -> Result<(), Diagnostic> {
    let _ = run(AppConfig { input })?;
    Ok(())
}

fn cmd_render(
    input: PathBuf,
    format: OutputFormat,
    annotated: bool,
    output: Option<PathBuf>,
) -> Result<(), Diagnostic> {
    let input_path = resolve_input_path(&input)?;
    let program = load_dir_program(&input_path)?;
    let graph = build_dir_graph(&program);

    match format {
        OutputFormat::Dot => {
            let dot = if annotated {
                render_dot_annotated(&graph)
            } else {
                render_dot(&graph)
            };
            write_output(dot, output)?;
        }
        OutputFormat::Json => {
            let json = render_json(&graph).map_err(|e| Diagnostic::message(e.to_string()))?;
            write_output(json, output)?;
        }
        OutputFormat::Svg => {
            return Err(Diagnostic::message(
                "SVG rendering is not implemented in v0.1",
            ));
        }
    }

    Ok(())
}

fn write_output(contents: String, output: Option<PathBuf>) -> Result<(), Diagnostic> {
    if let Some(path) = output {
        std::fs::write(&path, contents).map_err(|e| Diagnostic::Io {
            path,
            source: e,
        })?;
    } else {
        print!("{contents}");
    }
    Ok(())
}

fn report_error(err: Diagnostic) {
    eprintln!("error: {err}");
}