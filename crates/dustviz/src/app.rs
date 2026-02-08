// crates/dustviz/src/app.rs
//
// Top-level application orchestration for dustviz.
//
// Responsibilities:
// - accept resolved CLI arguments
// - load external artifacts
// - prepare data for later graph / render stages
//
// At v0.1, this file only wires input loading and basic flow.
// No graph construction or rendering is performed yet.

use std::path::PathBuf;

use crate::input::{load_dir_program, resolve_input_path};
use crate::model::ir::DirProgram;
use crate::util::diagnostics::Diagnostic;

/// Application configuration derived from CLI arguments.
pub struct AppConfig {
    pub input: PathBuf,
}

/// Run the dustviz application.
///
/// This function is intentionally small and explicit so that:
/// - errors propagate cleanly
/// - later stages (graph, render) can be added without refactoring
pub fn run(config: AppConfig) -> Result<DirProgram, Diagnostic> {
    let input_path = resolve_input_path(&config.input)?;
    let program = load_dir_program(&input_path)?;

    Ok(program)
}