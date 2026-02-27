// File: mod.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Model module - DIR model types for visualization.
//   Submodules: constraints, ir, trace.

pub mod constraints;
pub mod ir;
pub mod trace;

pub use constraints::{
    ConstraintAttr, ConstraintEdge, ConstraintEdgeKind, ConstraintId, ConstraintNode,
    ConstraintNodeKind, ConstraintsDocument, IrRef,
};
pub use ir::{
    DirBind, DirClause, DirConstrain, DirEffect, DirField, DirForge, DirLet, DirLit, DirParam,
    DirProc, DirProgram, DirProve, DirReturn, DirShape, DirStmt, DirUses,
};
pub use trace::{TraceDocument, TraceEvent, TraceEventKind};
