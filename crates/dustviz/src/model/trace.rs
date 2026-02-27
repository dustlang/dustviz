// File: trace.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Trace/event model for dustviz.
//   Trace files are JSON Lines (*.trace.jsonl) representing ordered events.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceDocument {
    pub events: Vec<TraceEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub index: usize,
    pub kind: TraceEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TraceEventKind {
    EnterProc {
        proc: String,
    },
    StmtExec {
        proc: String,
        stmt_index: usize,
    },
    ConstraintAdded {
        id: String,
    },
    ConstraintFailed {
        id: String,
        #[serde(default)]
        reason: Option<String>,
    },
}
