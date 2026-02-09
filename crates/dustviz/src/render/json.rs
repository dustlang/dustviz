// crates/dustviz/src/render/json.rs
//
// JSON renderer for dustviz graphs.
//
// v0.1 goal:
// - Deterministic JSON output for the internal `Graph` type.
// - Suitable for programmatic consumption (frontends, web viewers, etc.).
//
// Output schema (stable for v0.1):
// {
//   "nodes": [ { "id": <u32>, "kind": "<string>", "label": "<string>" }, ... ],
//   "edges": [ { "id": <u32>, "kind": "<string>", "from": <u32>, "to": <u32> }, ... ]
// }

use serde::Serialize;

use crate::graph::{EdgeKind, Graph, NodeKind};

#[derive(Debug, Serialize)]
pub struct GraphJson {
    pub nodes: Vec<NodeJson>,
    pub edges: Vec<EdgeJson>,
}

#[derive(Debug, Serialize)]
pub struct NodeJson {
    pub id: u32,
    pub kind: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct EdgeJson {
    pub id: u32,
    pub kind: String,
    pub from: u32,
    pub to: u32,
}

pub fn render_json(graph: &Graph) -> Result<String, serde_json::Error> {
    let nodes = graph
        .nodes
        .iter()
        .map(|n| {
            let (kind, label) = node_kind_and_label(&n.kind);
            NodeJson {
                id: n.id,
                kind,
                label,
            }
        })
        .collect();

    let edges = graph
        .edges
        .iter()
        .map(|e| EdgeJson {
            id: e.id,
            kind: edge_kind_label(e.kind).to_string(),
            from: e.from,
            to: e.to,
        })
        .collect();

    let doc = GraphJson { nodes, edges };
    serde_json::to_string_pretty(&doc)
}

fn node_kind_and_label(kind: &NodeKind) -> (String, String) {
    match kind {
        NodeKind::Program => ("Program".to_string(), "Program".to_string()),
        NodeKind::Forge { name } => ("Forge".to_string(), name.clone()),
        NodeKind::Shape { name } => ("Shape".to_string(), name.clone()),
        NodeKind::Proc { regime, name } => ("Proc".to_string(), format!("[{}] {}", regime, name)),
        NodeKind::Uses { resource } => ("Uses".to_string(), resource.clone()),
        NodeKind::Bind { source, target } => ("Bind".to_string(), format!("{} -> {}", source, target)),
        NodeKind::Clause { key, op, value } => {
            ("Clause".to_string(), format!("{} {} {}", key, op, value))
        }
        NodeKind::Stmt { label } => ("Stmt".to_string(), label.clone()),
    }
}

fn edge_kind_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Next => "next",
        EdgeKind::Uses => "uses",
        EdgeKind::Clause => "clause",
    }
}