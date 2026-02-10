// crates/dustviz/src/model/mod.rs

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
