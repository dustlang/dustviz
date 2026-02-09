// crates/dustviz/src/input/load_constraints.rs
//
// Loader for constraint overlay artifacts.
//
// This module mirrors `load_dir_program` but targets constraint documents.
// It performs:
// - path canonicalization
// - file loading
// - JSON deserialization into the typed constraint model
//
// No semantic validation or graph merging is done here.

use std::path::Path;

use crate::model::constraints::ConstraintsDocument;
use crate::util::diagnostics::Diagnostic;
use crate::util::fs;

pub fn load_constraints(path: &Path) -> Result<ConstraintsDocument, Diagnostic> {
    let canonical = fs::canonicalize(path)?;
    let contents = fs::read_to_string(&canonical)?;

    serde_json::from_str::<ConstraintsDocument>(&contents).map_err(|e| Diagnostic::Json {
        path: canonical,
        source: e,
    })
}