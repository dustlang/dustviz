// crates/dustviz/src/input/discover.rs
//
// Input discovery for dustviz.
//
// At v0.1, discovery is intentionally minimal and explicit.
// No implicit searching or convention-based lookup is performed.
//
// This module exists to provide a stable place for future expansion
// (e.g. multiple inputs, directories, or manifest-based discovery)
// without contaminating loader logic.

use std::path::{Path, PathBuf};

use crate::util::diagnostics::Diagnostic;

/// Resolve a user-supplied input path.
///
/// For now, this simply validates that the path exists and returns it.
/// All higher-level semantics are handled elsewhere.
pub fn resolve_input_path(path: &Path) -> Result<PathBuf, Diagnostic> {
    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err(Diagnostic::message(format!(
            "input path does not exist: {}",
            path.display()
        )))
    }
}