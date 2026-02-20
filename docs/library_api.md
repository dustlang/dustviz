# Library API

Sources:

- `src/lib.rs`
- `src/app.rs`
- `src/cli.rs`
- re-export modules in `src/*/mod.rs`

## Crate Modules

`lib.rs` exports:

- `app`
- `cli`
- `graph`
- `input`
- `model`
- `render`
- `util`

## app

- `AppConfig { input: PathBuf }`
- `run(config) -> Result<DirProgram, Diagnostic>`

## cli

- `Cli`
- `Command` enum (`Parse`, `Render`)
- `OutputFormat` enum (`Dot`, `Json`, `Svg`)

## input public exports

- `resolve_input_path`
- `load_dir_program`
- `load_constraints`
- `load_trace`

## model public exports

IR:

- `DirProgram`, `DirForge`, `DirShape`, `DirField`
- `DirProc`, `DirParam`, `DirUses`, `DirLit`
- `DirStmt`, `DirLet`, `DirConstrain`, `DirProve`, `DirEffect`, `DirReturn`
- `DirBind`, `DirClause`

Constraints:

- `ConstraintsDocument`
- `ConstraintId`, `ConstraintNode`, `ConstraintNodeKind`
- `ConstraintEdge`, `ConstraintEdgeKind`
- `IrRef`, `ConstraintAttr`

Trace:

- `TraceDocument`, `TraceEvent`, `TraceEventKind`

## graph public exports

From `graph/mod.rs`:

- `build_dir_graph`
- `focus_graph`
- `overlay_constraints`
- `link_constraint_ir_refs`
- `overlay_trace`, `TraceOverlay`
- graph types: `Graph`, `Node`, `Edge`, `NodeKind`, `EdgeKind`, `Attr`, `NodeId`, `EdgeId`

## render public exports

- `render_dot`
- `render_dot_annotated`
- `render_json`
- `render_svg`

## util

- `util::diagnostics::Diagnostic`
- `util::fs::{read_to_string, canonicalize}`

## Binary Entrypoint Contract

`src/main.rs` composes public APIs in this order:

- parse path uses `app::run`
- render path uses `input`, `graph`, and `render` modules directly
- all failures are reported as diagnostics
