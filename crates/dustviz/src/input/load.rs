// File: load.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Loader for DIR Program JSON artifacts.
//   Reads files and parses JSON into typed DIR model.
//   No validation beyond syntactic correctness.

use crate::model::ir::Program;

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
