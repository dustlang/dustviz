# Architecture

## Pipeline

`dustviz` follows a strict pipeline:

`CLI -> Input -> Model -> Graph -> Render`

## Module Layout

Workspace crate: `crates/dustviz`

- `src/main.rs`: CLI entrypoint and command dispatch.
- `src/app.rs`: top-level orchestration for `parse` path.
- `src/cli.rs`: clap command/flag definitions.
- `src/input/*`: path resolution and artifact loading.
- `src/model/*`: typed data models for IR, constraints, and trace events.
- `src/graph/*`: graph construction and overlays.
- `src/render/*`: backend renderers (DOT, JSON, SVG).
- `src/util/*`: diagnostics and filesystem helpers.

## Runtime Flow

### Parse Command

1. Resolve input path (`resolve_input_path`).
2. Load DIR program JSON (`load_dir_program`).
3. Build graph (`build_dir_graph`) for structural verification.
4. Return success or diagnostic error.

### Render Command

1. Resolve and load DIR program.
2. Build base IR graph.
3. Optional constraints overlay:
   - `overlay_constraints`
   - `link_constraint_ir_refs`
4. Optional trace overlay:
   - `overlay_trace`
5. Optional focused subgraph (`focus_graph`) when `--focus` is set.
6. Render via selected backend (`dot|json|svg`).

## Determinism Boundaries

Determinism is implemented by:

- stable node and edge ids from insertion order,
- sorted attribute tooltip rendering in DOT annotated mode,
- `BTreeMap` for JSON attributes,
- snapshot tests over fixture artifacts.

SVG output additionally depends on Graphviz behavior/version.
