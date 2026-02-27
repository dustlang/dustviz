// File: diagnostics.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Diagnostic error types for dustviz.

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Diagnostic {
    #[error("I/O error while reading {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse JSON in {path}: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("{0}")]
    Message(String),
}

impl Diagnostic {
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}
