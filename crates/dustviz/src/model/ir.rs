// File: ir.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   v0.1 DIR Program model (JSON) for dustviz.
//   Minimal strongly-typed representation matching program.dir.json.
//   Visualization model, not semantic authority.

use serde::Deserialize;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirProgram {
    pub forges: Vec<DirForge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirForge {
    pub name: String,
    pub shapes: Vec<DirShape>,
    pub procs: Vec<DirProc>,
    pub binds: Vec<DirBind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirShape {
    pub name: String,
    pub fields: Vec<DirField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirField {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirProc {
    pub regime: String,
    pub name: String,
    pub params: Vec<DirParam>,
    pub uses: Vec<DirUses>,
    #[serde(default)]
    pub ret: Option<String>,
    #[serde(default)]
    pub qualifiers: Vec<String>,
    pub body: Vec<DirStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirParam {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirUses {
    pub resource: String,
    pub args: Vec<(String, DirLit)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum DirLit {
    Int(i64),
    Bool(bool),
    String(String),
}

// Externally-tagged statement enum:
//
// { "Effect": { ... } }
// { "Return": { ... } }
// etc.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum DirStmt {
    Let(DirLet),
    Constrain(DirConstrain),
    Prove(DirProve),
    Effect(DirEffect),
    Return(DirReturn),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirLet {
    pub name: String,
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirConstrain {
    pub predicate: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirProve {
    pub name: String,
    pub from: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirEffect {
    pub kind: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirReturn {
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirBind {
    pub source: String,
    pub target: String,
    pub contract: Vec<DirClause>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DirClause {
    pub key: String,
    pub op: String,
    pub value: String,
}
