// crates/dustviz/src/model/constraints.rs
//
// Constraint model for dustviz.
//
// This module defines a minimal, typed representation of constraint artifacts
// that can be overlaid onto the core IR graph.
//
// Semantics are intentionally opaque:
// - dustviz does not interpret or validate constraints
// - all strings are treated as labels
//
// v0.2 scope:
// - constraint nodes
// - constraint edges
// - optional references into IR (by proc name / stmt index / symbol)

use serde::Deserialize;

/// Root constraint document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConstraintsDocument {
    pub nodes: Vec<ConstraintNode>,
    pub edges: Vec<ConstraintEdge>,
}

/// Unique identifier for a constraint node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ConstraintId(pub String);

/// A constraint node.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConstraintNode {
    pub id: ConstraintId,
    pub kind: ConstraintNodeKind,
    pub label: String,

    /// Optional reference into the IR.
    #[serde(default)]
    pub ir_ref: Option<IrRef>,

    /// Arbitrary metadata.
    #[serde(default)]
    pub attrs: Vec<ConstraintAttr>,
}

/// Kinds of constraint nodes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintNodeKind {
    Predicate,
    Rule,
    Obligation,
    Witness,
    Proof,
    Diagnostic,
}

/// A directed edge between two constraint nodes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConstraintEdge {
    pub from: ConstraintId,
    pub to: ConstraintId,
    pub kind: ConstraintEdgeKind,

    #[serde(default)]
    pub attrs: Vec<ConstraintAttr>,
}

/// Kinds of constraint edges.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintEdgeKind {
    DependsOn,
    Justifies,
    Requires,
    Blocks,
    Produces,
    RefersTo,
}

/// Optional reference into the IR graph.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct IrRef {
    /// Procedure name (if applicable).
    #[serde(default)]
    pub proc: Option<String>,

    /// Statement index within the procedure body (if applicable).
    #[serde(default)]
    pub stmt_index: Option<usize>,

    /// Symbol name or identifier (optional).
    #[serde(default)]
    pub symbol: Option<String>,
}

/// Arbitrary key/value metadata for constraints.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConstraintAttr {
    pub key: String,
    pub value: String,
}