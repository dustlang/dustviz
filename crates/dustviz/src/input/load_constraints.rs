// File: load_constraints.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Loader for constraint overlay artifacts.
//   Mirrors load_dir_program but targets constraint documents.

use crate::model::constraints::ConstraintsDocument;
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
