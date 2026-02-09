// crates/dustviz/src/app.rs
//
// Top-level application orchestration for dustviz.
//
// Responsibilities (v0.1):
// - accept resolved CLI arguments
// - load external DIR program artifact
// - build an in-memory graph representation
//
// Rendering is intentionally NOT performed here yet.

use std::path::PathBuf;

use crate::graph::build_dir_graph;
use crate::input::{load_dir_program, resolve_input_path};
use crate::model::ir::DirProgram;
use crate::util::diagnostics::Diagnostic;

/// Application configuration derived from CLI arguments.
pub struct AppConfig {
    pub input: PathBuf,
}

/// Run the dustviz application.
///
/// v0.1 behavior:
/// - load DIR program
/// - build graph
/// - return the parsed program (graph is built for side-effect readiness)
///
/// The graph will be surfaced in later steps (rendering / inspection).
pub fn run(config: AppConfig) -> Result<DirProgram, Diagnostic> {
    let input_path = resolve_input_path(&config.input)?;
    let program = load_dir_program(&input_path)?;

    // Build the graph eagerly to validate structural construction.
    // Result is intentionally unused for now.
    let _graph = build_dir_graph(&program);

    Ok(program)
}