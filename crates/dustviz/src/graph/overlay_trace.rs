// crates/dustviz/src/graph/overlay_trace.rs
//
// Overlay trace events onto an existing graph.
//
// v0.3 behavior:
// - Each trace event becomes a graph node (TraceEvent).
// - If the event maps to an IR or constraint node, add a Trace edge.
// - Track focus nodes touched by the trace for optional focus rendering.

use std::collections::{HashMap, HashSet};

use crate::graph::{Attr, EdgeKind, Graph, NodeId, NodeKind};
use crate::model::trace::{TraceDocument, TraceEventKind};

pub struct TraceOverlay {
    pub focus_nodes: HashSet<NodeId>,
}

pub fn overlay_trace(graph: &mut Graph, trace: &TraceDocument) -> TraceOverlay {
    let proc_name_to_id = index_procs(graph);
    let stmt_index_by_proc = index_stmts_by_proc(graph, &proc_name_to_id);
    let constraint_id_to_node = index_constraints(graph);

    let mut focus_nodes: HashSet<NodeId> = HashSet::new();

    for event in &trace.events {
        let (label, attrs, target) = describe_event(
            event.index,
            &event.kind,
            &proc_name_to_id,
            &stmt_index_by_proc,
            &constraint_id_to_node,
        );

        let event_node_id = graph.add_node_with_attrs(NodeKind::TraceEvent { label }, attrs);
        focus_nodes.insert(event_node_id);

        if let Some(target_id) = target {
            let edge_id = graph.add_edge(EdgeKind::Trace, event_node_id, target_id);
            graph.add_edge_attr(edge_id, "trace_event", event.index.to_string());
            focus_nodes.insert(target_id);
        }
    }

    TraceOverlay { focus_nodes }
}

fn describe_event(
    index: usize,
    event: &TraceEventKind,
    proc_name_to_id: &HashMap<String, NodeId>,
    stmt_index_by_proc: &HashMap<String, HashMap<usize, NodeId>>,
    constraint_id_to_node: &HashMap<String, NodeId>,
) -> (String, Vec<Attr>, Option<NodeId>) {
    let mut attrs = vec![
        Attr::new("kind", "trace_event"),
        Attr::new("event_index", index.to_string()),
    ];

    match event {
        TraceEventKind::EnterProc { proc } => {
            attrs.push(Attr::new("event_kind", "enter_proc"));
            attrs.push(Attr::new("proc", proc.clone()));
            let label = format!("Trace\nEnterProc {}", proc);
            let target = proc_name_to_id.get(proc).copied();
            (label, attrs, target)
        }
        TraceEventKind::StmtExec { proc, stmt_index } => {
            attrs.push(Attr::new("event_kind", "stmt_exec"));
            attrs.push(Attr::new("proc", proc.clone()));
            attrs.push(Attr::new("stmt_index", stmt_index.to_string()));
            let label = format!("Trace\nStmtExec {}:{}", proc, stmt_index);
            let target = stmt_index_by_proc
                .get(proc)
                .and_then(|m| m.get(stmt_index))
                .copied();
            (label, attrs, target)
        }
        TraceEventKind::ConstraintAdded { id } => {
            attrs.push(Attr::new("event_kind", "constraint_added"));
            attrs.push(Attr::new("constraint_id", id.clone()));
            let label = format!("Trace\nConstraintAdded {}", id);
            let target = constraint_id_to_node.get(id).copied();
            (label, attrs, target)
        }
        TraceEventKind::ConstraintFailed { id, reason } => {
            attrs.push(Attr::new("event_kind", "constraint_failed"));
            attrs.push(Attr::new("constraint_id", id.clone()));
            if let Some(r) = reason {
                attrs.push(Attr::new("reason", r.clone()));
            }
            let label = match reason {
                Some(r) => format!("Trace\nConstraintFailed {}\n{}", id, r),
                None => format!("Trace\nConstraintFailed {}", id),
            };
            let target = constraint_id_to_node.get(id).copied();
            (label, attrs, target)
        }
    }
}

fn index_procs(graph: &Graph) -> HashMap<String, NodeId> {
    let mut map = HashMap::new();

    for n in &graph.nodes {
        if let NodeKind::Proc { name, .. } = &n.kind {
            map.insert(name.clone(), n.id);
        }
    }

    map
}

fn index_stmts_by_proc(
    graph: &Graph,
    proc_name_to_id: &HashMap<String, NodeId>,
) -> HashMap<String, HashMap<usize, NodeId>> {
    let mut proc_id_to_name: HashMap<NodeId, String> = HashMap::new();
    for (name, &id) in proc_name_to_id {
        proc_id_to_name.insert(id, name.clone());
    }

    let mut out: HashMap<String, HashMap<usize, NodeId>> = HashMap::new();

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

fn index_constraints(graph: &Graph) -> HashMap<String, NodeId> {
    let mut map = HashMap::new();
    for node in &graph.nodes {
        if let Some(id) = get_attr_value(node.attrs.as_slice(), "constraint_id") {
            map.insert(id.to_string(), node.id);
        }
    }
    map
}

fn get_attr_value<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str> {
    for a in attrs {
        if a.key == key {
            return Some(a.value.as_str());
        }
    }
    None
}
