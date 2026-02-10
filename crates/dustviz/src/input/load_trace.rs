// crates/dustviz/src/input/load_trace.rs
//
// Loader for trace JSONL (*.trace.jsonl) artifacts.
//
// Each non-empty line must be a JSON object matching TraceEventKind.

use std::path::Path;

use crate::model::trace::{TraceDocument, TraceEvent, TraceEventKind};
use crate::util::diagnostics::Diagnostic;
use crate::util::fs;

pub fn load_trace(path: &Path) -> Result<TraceDocument, Diagnostic> {
    let canonical = fs::canonicalize(path)?;
    let contents = fs::read_to_string(&canonical)?;

    let mut events = Vec::new();

    for (line_idx, raw) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let kind = serde_json::from_str::<TraceEventKind>(line).map_err(|e| {
            Diagnostic::message(format!(
                "failed to parse JSON in {} at line {}: {}",
                canonical.display(),
                line_no,
                e
            ))
        })?;

        events.push(TraceEvent {
            index: events.len(),
            kind,
        });
    }

    Ok(TraceDocument { events })
}
