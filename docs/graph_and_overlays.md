# Graph and Overlays

Sources:

- `src/graph/build.rs`
- `src/graph/overlay_constraints.rs`
- `src/graph/link_ir_refs.rs`
- `src/graph/overlay_trace.rs`
- `src/graph/focus.rs`

## Core Graph Types

- `NodeId = u32`
- `EdgeId = u32`
- `Attr { key, value }`
- `Graph { nodes, edges }`
- `Node { id, kind, attrs }`
- `Edge { id, kind, from, to, attrs }`

### NodeKind

- `Program`
- `Forge { name }`
- `Shape { name }`
- `Proc { regime, name }`
- `Uses { resource }`
- `Bind { source, target }`
- `Clause { key, op, value }`
- `Stmt { label }`
- `TraceEvent { label }`

### EdgeKind

- `Contains`
- `Next`
- `Uses`
- `Clause`
- `Trace`

## Base Graph Construction

`build_dir_graph(program)` builds the base graph from DIR model.

Construction details:

1. Adds one `Program` node.
2. Adds forge nodes and `contains` edges.
3. Adds shape nodes under forges.
4. Adds proc nodes under forges.
5. Adds uses nodes from each proc (`uses` edges).
6. Adds statement nodes from proc body (`contains` edges).
7. Adds `next` edges between consecutive statements.
8. Adds bind nodes and clause nodes (`clause` edges from bind to clause).

IDs are assigned by insertion order and remain stable for fixed input traversal.

## Constraint Overlay

`overlay_constraints(graph, constraints)` mutates graph and returns:

- `HashMap<String, NodeId>` mapping constraint id string to node id.

Behavior:

- each constraint node is added as `NodeKind::Clause` with:
  - `key = "constraint"`
  - `op = <constraint label>`
  - `value = <Debug of constraint kind>`
- node attributes from constraint attrs are copied.
- synthetic attrs are added:
  - `constraint_id`
  - `constraint_kind`

Constraint edge kind mapping to `EdgeKind`:

- `DependsOn -> Uses`
- `Justifies -> Clause`
- `Requires -> Contains`
- `Blocks -> Next`
- `Produces -> Uses`
- `RefersTo -> Clause`

## IR Reference Linking

`link_constraint_ir_refs(graph, constraints, constraint_node_map)` adds reference edges from constraint nodes to IR nodes.

Resolution logic:

1. Index proc nodes by proc name.
2. Index stmt nodes by `(proc, stmt_index)` using:
   - `Contains` proc->stmt edges,
   - stmt attr key `index`.
3. For each constraint with `ir_ref`:
   - if `stmt_index` resolves, link constraint node to stmt node,
   - otherwise link to proc node.

Reference edges are `EdgeKind::Clause` with attrs such as:

- `link=ir_ref`
- `target=stmt|proc`
- `proc=<name>`
- optional `stmt_index`
- optional `symbol`

## Trace Overlay

`overlay_trace(graph, trace)` mutates graph and returns:

- `TraceOverlay { focus_nodes: HashSet<NodeId> }`

Behavior:

- each trace event becomes a `NodeKind::TraceEvent` node,
- optional `Trace` edge is added to resolved target node,
- both event node and target node are added to `focus_nodes`.

Event resolution targets:

- `EnterProc`: proc node by name,
- `StmtExec`: stmt node by proc + stmt index,
- `ConstraintAdded` / `ConstraintFailed`: constraint node via `constraint_id` attr.

## Focused Subgraph

`focus_graph(graph, focus_nodes)` returns a new graph with:

- only nodes where node id is in `focus_nodes`,
- only edges where both `from` and `to` ids are in `focus_nodes`.

This is used by CLI `--focus` rendering mode.
