// crates/dustviz/src/render/dot.rs
//
// Graphviz DOT renderer for dustviz graphs.
//
// v0.1 goal:
// - Deterministic DOT output for the internal `Graph` type.
// - No external `dot` invocation; this only emits text.
//
// Notes:
// - Node IDs are stable (assigned by insertion order).
// - We keep labels human-readable and escape strings for DOT safety.

use std::fmt::Write as _;

use crate::graph::{EdgeKind, Graph, NodeKind};

pub fn render_dot(graph: &Graph) -> String {
    let mut out = String::new();

    writeln!(&mut out, "digraph dustviz {{").unwrap();
    writeln!(&mut out, "  rankdir=LR;").unwrap();
    writeln!(&mut out, "  node [shape=box, fontname=\"monospace\"];").unwrap();
    writeln!(&mut out, "  edge [fontname=\"monospace\"];").unwrap();
    writeln!(&mut out).unwrap();

    // Nodes
    for n in &graph.nodes {
        let (label, shape) = node_label_and_shape(&n.kind);
        let label = dot_escape(&label);

        writeln!(
            &mut out,
            "  n{} [label=\"{}\", shape={}];",
            n.id, label, shape
        )
        .unwrap();
    }

    writeln!(&mut out).unwrap();

    // Edges
    for e in &graph.edges {
        let label = edge_label(e.kind);
        let label = dot_escape(label);

        writeln!(
            &mut out,
            "  n{} -> n{} [label=\"{}\"];",
            e.from, e.to, label
        )
        .unwrap();
    }

    writeln!(&mut out, "}}").unwrap();
    out
}

fn node_label_and_shape(kind: &NodeKind) -> (String, &'static str) {
    match kind {
        NodeKind::Program => ("Program".to_string(), "oval"),
        NodeKind::Forge { name } => (format!("Forge\n{}", name), "folder"),
        NodeKind::Shape { name } => (format!("Shape\n{}", name), "box3d"),
        NodeKind::Proc { regime, name } => (format!("Proc [{}]\n{}", regime, name), "component"),
        NodeKind::Uses { resource } => (format!("Uses\n{}", resource), "note"),
        NodeKind::Bind { source, target } => (format!("Bind\n{} → {}", source, target), "diamond"),
        NodeKind::Clause { key, op, value } => {
            (format!("Clause\n{} {} {}", key, op, value), "parallelogram")
        }
        NodeKind::Stmt { label } => (format!("Stmt\n{}", label), "box"),
    }
}

fn edge_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Next => "next",
        EdgeKind::Uses => "uses",
        EdgeKind::Clause => "clause",
    }
}

fn dot_escape(s: &str) -> String {
    // Minimal DOT string escaping: backslash, quote, and newlines.
    // Graphviz accepts \n for newlines in labels.
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(ch),
        }
    }
    out
}