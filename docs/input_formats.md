# Input Formats

This document describes how `dustviz` currently loads external artifacts.

## Path Resolution

Source: `src/input/discover.rs`

- `resolve_input_path(path)` only checks existence.
- No implicit discovery or directory search is performed.

## DIR Program JSON

Loader: `src/input/load.rs`
Model: `src/model/ir.rs`

Parsed into:

- `DirProgram`
- `DirForge`
- `DirShape` / `DirField`
- `DirProc` / `DirParam` / `DirUses`
- `DirStmt` variants: `Let`, `Constrain`, `Prove`, `Effect`, `Return`
- `DirBind` / `DirClause`

Notes:

- Strings such as type names, expressions, predicates, and payloads are opaque labels at visualization layer.
- Deserialization is strict JSON parse with serde.

## Constraints JSON

Loader: `src/input/load_constraints.rs`
Model: `src/model/constraints.rs`

Parsed into:

- `ConstraintsDocument { nodes, edges }`
- `ConstraintNode` and `ConstraintEdge`
- `ConstraintNodeKind` and `ConstraintEdgeKind`
- optional `IrRef` linking hints (`proc`, `stmt_index`, `symbol`)

Constraint kinds (snake_case in JSON):

- predicate, rule, obligation, witness, proof, diagnostic

Edge kinds (snake_case in JSON):

- depends_on, justifies, requires, blocks, produces, refers_to

## Trace JSONL

Loader: `src/input/load_trace.rs`
Model: `src/model/trace.rs`

Rules:

- input is line-oriented JSON (JSONL),
- empty lines are ignored,
- each non-empty line must deserialize as `TraceEventKind`.

Supported `kind` values:

- `enter_proc` with field `proc`
- `stmt_exec` with fields `proc`, `stmt_index`
- `constraint_added` with field `id`
- `constraint_failed` with fields `id`, optional `reason`

Each parsed event is assigned a sequential `index` in load order.

## Loader Error Semantics

- path and read errors map to `Diagnostic::Io`.
- JSON parse failures map to `Diagnostic::Json` (DIR/constraints) or `Diagnostic::Message` (trace line parse with line number context).
