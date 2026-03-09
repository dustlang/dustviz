# dustviz

`dustviz` is now a Dust-native visualization pipeline for external IR/constraint/trace artifacts.

It preserves the original architecture boundaries:

`CLI -> Input -> Graph -> Render`

and keeps the same command contract:

- `parse --input <dir.json>`
- `render --input <dir.json> [--constraints <constraints.json>] [--trace <trace.jsonl>] [--format dot|json|svg] [--annotated] [--focus] [--output <path>]`

## Dust-Native Layout

- Workspace manifest: `State.toml`
- Sources: `src/*.ds`
- Entry point: `src/main.ds`
- Deterministic self-tests: `src/viz_tests.ds`
- Fixtures: `tests/fixtures/`

## Build Status

DustViz is now authored in Dust (`.ds`) using forge/proc module style used across the workspace.

Current `dust` compiler builds in this environment still target top-level `K main` grammar and do not fully accept forge/proc namespace syntax (`proc K::...`). DustViz source is prepared for the forge-style runtime/tooling path.

## Notes

- This migration removes Rust crate wiring from `dustviz`.
- The Dust implementation keeps deterministic orchestration and CLI shape while using Dust-side tokenized artifact representations.
