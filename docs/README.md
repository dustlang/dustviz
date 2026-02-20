# dustviz Documentation

This directory contains complete project documentation for `dustviz`.

## Documentation Map

- `getting_started.md`: prerequisites, build, and first commands.
- `architecture.md`: end-to-end pipeline and module boundaries.
- `cli_reference.md`: command and flag reference.
- `input_formats.md`: expected external artifact shapes.
- `graph_and_overlays.md`: graph model, constraint overlay, trace overlay, and focus behavior.
- `rendering.md`: DOT, JSON, and SVG render contracts.
- `library_api.md`: public Rust API and module export surface.
- `diagnostics.md`: error model and failure semantics.
- `testing.md`: test layout, fixtures, and snapshot workflow.

## Project Scope Summary

`dustviz` is a visualization tool for external artifacts. It:

- loads DIR/constraint/trace artifacts,
- builds an internal graph,
- renders deterministic outputs.

It does not define language semantics, compile programs, or execute programs.
