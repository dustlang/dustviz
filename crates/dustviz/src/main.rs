// crates/dustviz/src/main.rs

use std::path::PathBuf;
use std::process;

use clap::Parser;

use dustviz::app::{run, AppConfig};
use dustviz::cli::{Cli, Command, OutputFormat};
use dustviz::graph::{
    build_dir_graph, focus_graph, link_constraint_ir_refs, overlay_constraints, overlay_trace,
};
use dustviz::input::{load_constraints, load_dir_program, load_trace, resolve_input_path};
use dustviz::render::{render_dot, render_dot_annotated, render_json, render_svg};
use dustviz::util::diagnostics::Diagnostic;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Parse { input } => cmd_parse(input),
        Command::Render {
            input,
            constraints,
            trace,
            format,
            annotated,
            focus,
            output,
        } => cmd_render(input, constraints, trace, format, annotated, focus, output),
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
    constraints: Option<PathBuf>,
    trace: Option<PathBuf>,
    format: OutputFormat,
    annotated: bool,
    focus: bool,
    output: Option<PathBuf>,
) -> Result<(), Diagnostic> {
    let input_path = resolve_input_path(&input)?;
    let program = load_dir_program(&input_path)?;
    let mut graph = build_dir_graph(&program);

    if let Some(constraints_path) = constraints {
        let constraints_path = resolve_input_path(&constraints_path)?;
        let constraints_doc = load_constraints(&constraints_path)?;

        let map = overlay_constraints(&mut graph, &constraints_doc);
        link_constraint_ir_refs(&mut graph, &constraints_doc, &map);
    }

    let focus_nodes = if let Some(trace_path) = trace {
        let trace_path = resolve_input_path(&trace_path)?;
        let trace_doc = load_trace(&trace_path)?;
        let overlay = overlay_trace(&mut graph, &trace_doc);
        Some(overlay.focus_nodes)
    } else {
        None
    };

    let graph = if focus {
        let Some(nodes) = focus_nodes.as_ref() else {
            return Err(Diagnostic::message(
                "--focus requires a trace file via --trace",
            ));
        };
        focus_graph(&graph, nodes)
    } else {
        graph
    };

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
            let dot = if annotated {
                render_dot_annotated(&graph)
            } else {
                render_dot(&graph)
            };
            let svg = render_svg(&dot)?;
            write_output(svg, output)?;
        }
    }

    Ok(())
}

fn write_output(contents: String, output: Option<PathBuf>) -> Result<(), Diagnostic> {
    if let Some(path) = output {
        std::fs::write(&path, contents).map_err(|e| Diagnostic::Io { path, source: e })?;
    } else {
        print!("{contents}");
    }
    Ok(())
}

fn report_error(err: Diagnostic) {
    eprintln!("error: {err}");
}
