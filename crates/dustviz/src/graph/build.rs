// crates/dustviz/src/graph/build.rs
//
// Build an in-memory graph from a parsed DIR program.
//
// v0.1 goal:
// - Deterministically convert `DirProgram` into a graph structure that can later be rendered.
// - No external graph crate required yet (keeps build simple / operational in CI).
// - Stable node ordering and IDs derived from traversal order.
//
// This is a visualization graph:
// - Strings like `ty`, `expr`, and `predicate` are treated as opaque labels here.

use crate::model::ir::{
    DirBind, DirClause, DirForge, DirProc, DirProgram, DirShape, DirStmt, DirUses,
};

pub type NodeId = u32;
pub type EdgeId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    Contains,
    Next,
    Uses,
    Clause,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Program,
    Forge { name: String },
    Shape { name: String },
    Proc { regime: String, name: String },
    Uses { resource: String },
    Bind { source: String, target: String },
    Clause { key: String, op: String, value: String },
    Stmt { label: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: EdgeId,
    pub kind: EdgeKind,
    pub from: NodeId,
    pub to: NodeId,
}

#[derive(Debug, Default, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, kind: NodeKind) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(Node { id, kind });
        id
    }

    pub fn add_edge(&mut self, kind: EdgeKind, from: NodeId, to: NodeId) -> EdgeId {
        let id = self.edges.len() as EdgeId;
        self.edges.push(Edge { id, kind, from, to });
        id
    }
}

pub fn build_dir_graph(program: &DirProgram) -> Graph {
    let mut g = Graph::new();

    let program_id = g.add_node(NodeKind::Program);

    for forge in &program.forges {
        let forge_id = build_forge(&mut g, program_id, forge);
        build_forge_children(&mut g, forge_id, forge);
    }

    g
}

fn build_forge(g: &mut Graph, parent: NodeId, forge: &DirForge) -> NodeId {
    let forge_id = g.add_node(NodeKind::Forge {
        name: forge.name.clone(),
    });
    g.add_edge(EdgeKind::Contains, parent, forge_id);
    forge_id
}

fn build_forge_children(g: &mut Graph, forge_id: NodeId, forge: &DirForge) {
    for shape in &forge.shapes {
        let shape_id = build_shape(g, forge_id, shape);
        g.add_edge(EdgeKind::Contains, forge_id, shape_id);
    }

    for proc_ in &forge.procs {
        build_proc(g, forge_id, proc_);
    }

    for bind in &forge.binds {
        build_bind(g, forge_id, bind);
    }
}

fn build_shape(g: &mut Graph, _forge_id: NodeId, shape: &DirShape) -> NodeId {
    g.add_node(NodeKind::Shape {
        name: shape.name.clone(),
    })
}

fn build_proc(g: &mut Graph, forge_id: NodeId, proc_: &DirProc) -> NodeId {
    let proc_id = g.add_node(NodeKind::Proc {
        regime: proc_.regime.clone(),
        name: proc_.name.clone(),
    });
    g.add_edge(EdgeKind::Contains, forge_id, proc_id);

    for uses in &proc_.uses {
        let uses_id = build_uses(g, proc_id, uses);
        g.add_edge(EdgeKind::Uses, proc_id, uses_id);
    }

    let mut prev_stmt: Option<NodeId> = None;
    for stmt in &proc_.body {
        let stmt_id = build_stmt(g, proc_id, stmt);
        g.add_edge(EdgeKind::Contains, proc_id, stmt_id);

        if let Some(prev) = prev_stmt {
            g.add_edge(EdgeKind::Next, prev, stmt_id);
        }
        prev_stmt = Some(stmt_id);
    }

    proc_id
}

fn build_uses(g: &mut Graph, _proc_id: NodeId, uses: &DirUses) -> NodeId {
    g.add_node(NodeKind::Uses {
        resource: uses.resource.clone(),
    })
}

fn build_stmt(g: &mut Graph, _proc_id: NodeId, stmt: &DirStmt) -> NodeId {
    let label = match stmt {
        DirStmt::Let(s) => format!("Let {} = {}", s.name, s.expr),
        DirStmt::Constrain(s) => format!("Constrain {}", s.predicate),
        DirStmt::Prove(s) => format!("Prove {} from {}", s.name, s.from),
        DirStmt::Effect(s) => format!("Effect {} {}", s.kind, s.payload),
        DirStmt::Return(s) => format!("Return {}", s.expr),
    };

    g.add_node(NodeKind::Stmt { label })
}

fn build_bind(g: &mut Graph, forge_id: NodeId, bind: &DirBind) -> NodeId {
    let bind_id = g.add_node(NodeKind::Bind {
        source: bind.source.clone(),
        target: bind.target.clone(),
    });
    g.add_edge(EdgeKind::Contains, forge_id, bind_id);

    for clause in &bind.contract {
        let clause_id = build_clause(g, bind_id, clause);
        g.add_edge(EdgeKind::Clause, bind_id, clause_id);
    }

    bind_id
}

fn build_clause(g: &mut Graph, _bind_id: NodeId, clause: &DirClause) -> NodeId {
    g.add_node(NodeKind::Clause {
        key: clause.key.clone(),
        op: clause.op.clone(),
        value: clause.value.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::load_dir_program;
    use crate::model::ir::DirProgram;
    use std::path::PathBuf;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("minimal")
            .join("program.dir.json")
    }

    fn load_fixture() -> DirProgram {
        load_dir_program(&fixture_path()).expect("fixture parses")
    }

    #[test]
    fn builds_basic_program_forge_proc_stmt_structure() {
        let program = load_fixture();
        let graph = build_dir_graph(&program);

        // Node presence
        assert!(
            graph.nodes.iter().any(|n| matches!(n.kind, NodeKind::Program)),
            "expected a Program node"
        );

        assert!(
            graph.nodes.iter().any(|n| matches!(&n.kind, NodeKind::Forge { name } if name == "core")),
            "expected Forge(core)"
        );

        assert!(
            graph.nodes.iter().any(|n| matches!(&n.kind, NodeKind::Proc { name, .. } if name == "main")),
            "expected Proc(main)"
        );

        assert!(
            graph.nodes.iter().any(|n| matches!(&n.kind, NodeKind::Stmt { .. })),
            "expected at least one Stmt node"
        );

        // Contains edges exist
        assert!(
            graph.edges.iter().any(|e| e.kind == EdgeKind::Contains),
            "expected at least one Contains edge"
        );

        // Optional: ensure Program -> Forge and Forge -> Proc containment exists.
        let program_id = graph
            .nodes
            .iter()
            .find(|n| matches!(n.kind, NodeKind::Program))
            .map(|n| n.id)
            .expect("Program node id");

        let core_forge_id = graph
            .nodes
            .iter()
            .find(|n| matches!(&n.kind, NodeKind::Forge { name } if name == "core"))
            .map(|n| n.id)
            .expect("Forge(core) id");

        assert!(
            graph.edges.iter().any(|e| {
                e.kind == EdgeKind::Contains && e.from == program_id && e.to == core_forge_id
            }),
            "expected Program contains Forge(core)"
        );

        let main_proc_id = graph
            .nodes
            .iter()
            .find(|n| matches!(&n.kind, NodeKind::Proc { name, .. } if name == "main"))
            .map(|n| n.id)
            .expect("Proc(main) id");

        assert!(
            graph.edges.iter().any(|e| {
                e.kind == EdgeKind::Contains && e.from == core_forge_id && e.to == main_proc_id
            }),
            "expected Forge(core) contains Proc(main)"
        );
    }

    #[test]
    fn statement_next_edges_follow_body_order() {
        let program = load_fixture();
        let graph = build_dir_graph(&program);

        // Collect stmt node ids in insertion order (they're inserted in body order).
        let stmt_ids: Vec<u32> = graph
            .nodes
            .iter()
            .filter_map(|n| match &n.kind {
                NodeKind::Stmt { .. } => Some(n.id),
                _ => None,
            })
            .collect();

        // With the minimal fixture we expect at least 2 statements (Effect, Return).
        assert!(
            stmt_ids.len() >= 2,
            "expected at least two Stmt nodes in the minimal fixture"
        );

        // Verify a Next edge exists between the first two statement nodes.
        let a = stmt_ids[0];
        let b = stmt_ids[1];

        assert!(
            graph.edges.iter().any(|e| e.kind == EdgeKind::Next && e.from == a && e.to == b),
            "expected Next edge between first and second statement nodes"
        );
    }
}