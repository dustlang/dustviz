// crates/dustviz/src/model/mod.rs

pub mod ir;

pub use ir::{
    DirBind, DirClause, DirConstrain, DirEffect, DirField, DirForge, DirLet, DirLit, DirParam,
    DirProc, DirProgram, DirProve, DirReturn, DirShape, DirStmt, DirUses,
};