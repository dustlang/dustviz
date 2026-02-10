// crates/dustviz/src/render/json.rs
//
// JSON renderer for dustviz graphs.
//
// v0.1+annotations goal:
// - Deterministic JSON output for the internal `Graph` type.
// - Include node/edge annotations as `attrs`.
//
// Output schema (stable from this point):
// {
//   "nodes": [
//     { "id": <u32>, "kind": "<string>", "label": "<string>", "attrs": { ... } },
//     ...
//   ],
//   "edges": [
//     { "id": <u32>, "kind": "<string>", "from": <u32>, "to": <u32>, "attrs": { ... } },
//     ...
//   ]
// }

use serde::Serialize;
use std::collections::BTreeMap;

use crate::graph::{Attr, EdgeKind, Graph, NodeKind};

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
    pub attrs: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct EdgeJson {
    pub id: u32,
    pub kind: String,
    pub from: u32,
    pub to: u32,
    pub attrs: BTreeMap<String, String>,
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
                attrs: attrs_map(&n.attrs),
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
            attrs: attrs_map(&e.attrs),
        })
        .collect();

    let doc = GraphJson { nodes, edges };
    serde_json::to_string_pretty(&doc)
}

fn attrs_map(attrs: &[Attr]) -> BTreeMap<String, String> {
    // Deterministic ordering in JSON via BTreeMap.
    // If duplicate keys occur, the last one wins.
    let mut map = BTreeMap::new();
    for a in attrs {
        map.insert(a.key.clone(), a.value.clone());
    }
    map
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
        NodeKind::TraceEvent { label } => ("TraceEvent".to_string(), label.clone()),
    }
}

fn edge_kind_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Next => "next",
        EdgeKind::Uses => "uses",
        EdgeKind::Clause => "clause",
        EdgeKind::Trace => "trace",
    }
}
