// File: focus.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Build focused subgraph containing only selected nodes and edges.

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
