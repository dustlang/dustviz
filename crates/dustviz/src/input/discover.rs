// File: discover.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Input discovery for dustviz.
//   v0.1: Minimal and explicit, no implicit searching.
//   Future expansion point for convention-based lookup.

use std::path::PathBuf;
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
