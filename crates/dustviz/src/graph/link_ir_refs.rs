// crates/dustviz/src/graph/link_ir_refs.rs
//
// Link constraint nodes to IR nodes using `ir_ref` when possible.
//
// This module adds edges from constraint nodes (already overlaid into the graph)
// to IR nodes (Proc / Stmt) when resolvable.
//
// Design constraints:
// - No semantic validation.
// - Deterministic linking based on stable traversal.
// - Does not change existing IR-only snapshots unless constraints are supplied.
// - Uses existing EdgeKind::Clause for reference edges, with attrs to distinguish.
//
// Expected usage order:
// 1) build_dir_graph(&DirProgram) -> Graph
// 2) overlay_constraints(&mut Graph, &ConstraintsDocument) -> constraint_id_to_node_id map
// 3) link_constraint_ir_refs(&mut Graph, &ConstraintsDocument, &map)

use std::collections::HashMap;

use crate::graph::{EdgeKind, Graph, NodeId, NodeKind};
use crate::model::constraints::ConstraintsDocument;

pub fn link_constraint_ir_refs(
    graph: &mut Graph,
    constraints: &ConstraintsDocument,
    constraint_node_map: &HashMap<String, NodeId>,
) {
    let proc_name_to_id = index_procs(graph);
    let stmt_index_by_proc = index_stmts_by_proc(graph, &proc_name_to_id);

    for cnode in &constraints.nodes {
        let Some(ir_ref) = &cnode.ir_ref else { continue };
        let Some(&cnode_id) = constraint_node_map.get(&cnode.id.0) else { continue };

        let Some(proc_name) = ir_ref.proc.as_deref() else { continue };
        let Some(&proc_id) = proc_name_to_id.get(proc_name) else { continue };

        // If stmt_index is present, try to link to that stmt; otherwise link to proc.
        if let Some(stmt_index) = ir_ref.stmt_index {
            if let Some(stmt_map) = stmt_index_by_proc.get(proc_name) {
                if let Some(&stmt_id) = stmt_map.get(&stmt_index) {
                    let edge_id = graph.add_edge(EdgeKind::Clause, cnode_id, stmt_id);
                    graph.add_edge_attr(edge_id, "link", "ir_ref");
                    graph.add_edge_attr(edge_id, "target", "stmt");
                    graph.add_edge_attr(edge_id, "proc", proc_name.to_string());
                    graph.add_edge_attr(edge_id, "stmt_index", stmt_index.to_string());

                    if let Some(sym) = ir_ref.symbol.as_deref() {
                        graph.add_edge_attr(edge_id, "symbol", sym.to_string());
                    }

                    continue;
                }
            }
        }

        // Fallback: link to the proc if stmt resolution fails or stmt_index is absent.
        let edge_id = graph.add_edge(EdgeKind::Clause, cnode_id, proc_id);
        graph.add_edge_attr(edge_id, "link", "ir_ref");
        graph.add_edge_attr(edge_id, "target", "proc");
        graph.add_edge_attr(edge_id, "proc", proc_name.to_string());

        if let Some(sym) = ir_ref.symbol.as_deref() {
            graph.add_edge_attr(edge_id, "symbol", sym.to_string());
        }
    }
}

fn index_procs(graph: &Graph) -> HashMap<String, NodeId> {
    let mut map = HashMap::new();

    for n in &graph.nodes {
        if let NodeKind::Proc { name, .. } = &n.kind {
            // If duplicates exist, last wins. This is acceptable for visualization.
            map.insert(name.clone(), n.id);
        }
    }

    map
}

fn index_stmts_by_proc(
    graph: &Graph,
    proc_name_to_id: &HashMap<String, NodeId>,
) -> HashMap<String, HashMap<usize, NodeId>> {
    // Build reverse proc_id -> proc_name for deterministic association.
    let mut proc_id_to_name: HashMap<NodeId, String> = HashMap::new();
    for (name, &id) in proc_name_to_id {
        proc_id_to_name.insert(id, name.clone());
    }

    // For each proc, map stmt_index -> stmt_node_id.
    let mut out: HashMap<String, HashMap<usize, NodeId>> = HashMap::new();

    // We can resolve stmt membership by Contains edges from Proc -> Stmt
    // and stmt index via node attrs key="index".
    for e in &graph.edges {
        if e.kind != EdgeKind::Contains {
            continue;
        }

        let Some(proc_name) = proc_id_to_name.get(&e.from) else { continue };

        let stmt_node = match graph.nodes.get(e.to as usize) {
            Some(n) => n,
            None => continue,
        };

        if !matches!(stmt_node.kind, NodeKind::Stmt { .. }) {
            continue;
        }

        let Some(index_val) = get_attr_value(stmt_node.attrs.as_slice(), "index") else {
            continue;
        };

        let Ok(idx) = index_val.parse::<usize>() else { continue };

        out.entry(proc_name.clone())
            .or_default()
            .insert(idx, stmt_node.id);
    }

    out
}

fn get_attr_value<'a>(attrs: &'a [crate::graph::Attr], key: &str) -> Option<&'a str> {
    for a in attrs {
        if a.key == key {
            return Some(a.value.as_str());
        }
    }
    None
}
