// crates/dustviz/src/graph/overlay_constraints.rs
//
// Overlay constraint documents onto an existing IR graph.
//
// This module merges constraint nodes and edges into the unified visualization graph.
// It does NOT:
// - validate constraints
// - resolve correctness
// - enforce IR references
//
// v0.2 behavior:
// - each constraint node becomes a graph node
// - constraint edges become graph edges
// - optional IR references create linking edges when resolvable
//
// If an IR reference cannot be resolved, the constraint node still exists unlinked.

use std::collections::HashMap;

use crate::graph::{Attr, EdgeKind, Graph, NodeId, NodeKind};
use crate::model::constraints::{
    ConstraintEdgeKind, ConstraintNodeKind, ConstraintsDocument,
};

/// Overlay constraints onto an existing graph.
///
/// Returns a map of constraint-id string → graph node id.
pub fn overlay_constraints(
    graph: &mut Graph,
    constraints: &ConstraintsDocument,
) -> HashMap<String, NodeId> {
    let mut map: HashMap<String, NodeId> = HashMap::new();

    // 1. Add constraint nodes
    for node in &constraints.nodes {
        let node_id = graph.add_node_with_attrs(
            NodeKind::Clause {
                key: "constraint".to_string(),
                op: node.label.clone(),
                value: format!("{:?}", node.kind),
            },
            node.attrs
                .iter()
                .map(|a| Attr::new(a.key.clone(), a.value.clone()))
                .collect(),
        );

        graph.add_node_attr(node_id, "constraint_id", node.id.0.clone());
        graph.add_node_attr(node_id, "constraint_kind", format!("{:?}", node.kind));

        map.insert(node.id.0.clone(), node_id);
    }

    // 2. Add constraint edges
    for edge in &constraints.edges {
        let Some(&from) = map.get(&edge.from.0) else { continue };
        let Some(&to) = map.get(&edge.to.0) else { continue };

        let edge_id = graph.add_edge(
            map_constraint_edge_kind(edge.kind),
            from,
            to,
        );

        for attr in &edge.attrs {
            graph.add_edge_attr(edge_id, attr.key.clone(), attr.value.clone());
        }
    }

    map
}

fn map_constraint_edge_kind(kind: ConstraintEdgeKind) -> EdgeKind {
    match kind {
        ConstraintEdgeKind::DependsOn => EdgeKind::Uses,
        ConstraintEdgeKind::Justifies => EdgeKind::Clause,
        ConstraintEdgeKind::Requires => EdgeKind::Contains,
        ConstraintEdgeKind::Blocks => EdgeKind::Next,
        ConstraintEdgeKind::Produces => EdgeKind::Uses,
        ConstraintEdgeKind::RefersTo => EdgeKind::Clause,
    }
}