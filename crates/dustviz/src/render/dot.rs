// crates/dustviz/src/render/dot.rs
//
// Graphviz DOT renderer for dustviz graphs.
//
// v0.1 goal:
// - Deterministic DOT output for the internal `Graph` type.
// - No external `dot` invocation; this only emits text.
//
// v0.1+annotations:
// - Preserve the existing `render_dot()` output exactly (DOT snapshot stability).
// - Add `render_dot_annotated()` which surfaces node/edge annotations via Graphviz
//   `tooltip` attributes (labels remain unchanged).
//
// Notes:
// - Node IDs are stable (assigned by insertion order).
// - We keep labels human-readable and escape strings for DOT safety.

use std::fmt::Write as _;

use crate::graph::{Attr, EdgeKind, Graph, NodeKind};

pub fn render_dot(graph: &Graph) -> String {
    render_dot_inner(graph, DotMode::Plain)
}

/// Render DOT with annotation surfacing (tooltips).
///
/// This does not change visible labels; it adds `tooltip="..."` attributes
/// to nodes/edges when they have annotations.
pub fn render_dot_annotated(graph: &Graph) -> String {
    render_dot_inner(graph, DotMode::Annotated)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DotMode {
    Plain,
    Annotated,
}

fn render_dot_inner(graph: &Graph, mode: DotMode) -> String {
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

        match mode {
            DotMode::Plain => {
                writeln!(
                    &mut out,
                    "  n{} [label=\"{}\", shape={}];",
                    n.id, label, shape
                )
                .unwrap();
            }
            DotMode::Annotated => {
                if n.attrs.is_empty() {
                    writeln!(
                        &mut out,
                        "  n{} [label=\"{}\", shape={}];",
                        n.id, label, shape
                    )
                    .unwrap();
                } else {
                    let tooltip = dot_escape(&format_attrs_tooltip(&n.attrs));
                    writeln!(
                        &mut out,
                        "  n{} [label=\"{}\", shape={}, tooltip=\"{}\"];",
                        n.id, label, shape, tooltip
                    )
                    .unwrap();
                }
            }
        }
    }

    writeln!(&mut out).unwrap();

    // Edges
    for e in &graph.edges {
        let label = edge_label(e.kind);
        let label = dot_escape(label);

        match mode {
            DotMode::Plain => {
                writeln!(
                    &mut out,
                    "  n{} -> n{} [label=\"{}\"];",
                    e.from, e.to, label
                )
                .unwrap();
            }
            DotMode::Annotated => {
                if e.attrs.is_empty() {
                    writeln!(
                        &mut out,
                        "  n{} -> n{} [label=\"{}\"];",
                        e.from, e.to, label
                    )
                    .unwrap();
                } else {
                    let tooltip = dot_escape(&format_attrs_tooltip(&e.attrs));
                    writeln!(
                        &mut out,
                        "  n{} -> n{} [label=\"{}\", tooltip=\"{}\"];",
                        e.from, e.to, label, tooltip
                    )
                    .unwrap();
                }
            }
        }
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

fn format_attrs_tooltip(attrs: &[Attr]) -> String {
    // Deterministic order: (key, value) lexicographic by key then value.
    // Tooltip is multi-line: "key=value\nkey=value"
    let mut items: Vec<(&str, &str)> = attrs.iter().map(|a| (a.key.as_str(), a.value.as_str())).collect();
    items.sort_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));

    let mut s = String::new();
    for (i, (k, v)) in items.iter().enumerate() {
        if i > 0 {
            s.push('\n');
        }
        s.push_str(k);
        s.push('=');
        s.push_str(v);
    }
    s
}

fn dot_escape(s: &str) -> String {
    // Minimal DOT string escaping: backslash, quote, and newlines.
    // Graphviz accepts \n for newlines in labels/tooltips.
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