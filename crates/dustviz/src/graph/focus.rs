// crates/dustviz/src/graph/focus.rs
//
// Build a focused subgraph containing only selected nodes and edges.

use std::collections::HashSet;

use crate::graph::{Edge, Graph, Node, NodeId};

pub fn focus_graph(graph: &Graph, focus_nodes: &HashSet<NodeId>) -> Graph {
    let nodes: Vec<Node> = graph
        .nodes
        .iter()
        .filter(|n| focus_nodes.contains(&n.id))
        .cloned()
        .collect();

    let edges: Vec<Edge> = graph
        .edges
        .iter()
        .filter(|e| focus_nodes.contains(&e.from) && focus_nodes.contains(&e.to))
        .cloned()
        .collect();

    Graph { nodes, edges }
}
