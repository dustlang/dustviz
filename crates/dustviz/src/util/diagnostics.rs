// crates/dustviz/src/util/diagnostics.rs

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Diagnostic {
    #[error("I/O error while reading {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },

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