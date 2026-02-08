// crates/dustviz/src/input/load.rs
//
// Loader for DIR Program JSON artifacts.
//
// This module is responsible only for:
// - reading a file from disk
// - parsing JSON into the typed DIR model
//
// It performs no validation beyond syntactic correctness.

use std::path::Path;

use crate::model::ir::DirProgram;
use crate::util::diagnostics::Diagnostic;
use crate::util::fs;

pub fn load_dir_program(path: &Path) -> Result<DirProgram, Diagnostic> {
    let canonical = fs::canonicalize(path)?;
    let contents = fs::read_to_string(&canonical)?;

    serde_json::from_str::<DirProgram>(&contents).map_err(|e| Diagnostic::Json {
        path: canonical,
        source: e,
    })
}