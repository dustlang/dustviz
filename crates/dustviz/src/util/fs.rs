// crates/dustviz/src/util/fs.rs

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::util::diagnostics::Diagnostic;

pub fn read_to_string(path: &Path) -> Result<String, Diagnostic> {
    let mut file =
        File::open(path).map_err(|e| Diagnostic::Io { path: path.to_path_buf(), source: e })?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|e| Diagnostic::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

    Ok(buf)
}

pub fn canonicalize(path: &Path) -> Result<PathBuf, Diagnostic> {
    path.canonicalize().map_err(|e| Diagnostic::Io {
        path: path.to_path_buf(),
        source: e,
    })
}