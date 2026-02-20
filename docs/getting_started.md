# Getting Started

## Prerequisites

- Rust toolchain (stable)
- Cargo
- Optional for SVG output: Graphviz (`dot`) in `PATH`

## Build

From the `dustviz` workspace root:

```bash
cargo build --workspace
```

## Run Tests

```bash
cargo test --workspace --verbose
```

## CLI Basics

`dustviz` has two subcommands:

1. `parse`: load and parse a DIR program artifact.
2. `render`: render a graph from DIR input with optional overlays.

## Parse Example

```bash
cargo run -p dustviz -- parse --input crates/dustviz/tests/fixtures/minimal/program.dir.json
```

## Render Examples

Render DOT:

```bash
cargo run -p dustviz -- render \
  --input crates/dustviz/tests/fixtures/minimal/program.dir.json \
  --format dot
```

Render JSON:

```bash
cargo run -p dustviz -- render \
  --input crates/dustviz/tests/fixtures/minimal/program.dir.json \
  --format json
```

Render SVG (requires Graphviz):

```bash
cargo run -p dustviz -- render \
  --input crates/dustviz/tests/fixtures/minimal/program.dir.json \
  --format svg
```

Render with constraints and trace focus:

```bash
cargo run -p dustviz -- render \
  --input crates/dustviz/tests/fixtures/minimal/program.dir.json \
  --constraints crates/dustviz/tests/fixtures/constraints/minimal/constraints.json \
  --trace crates/dustviz/tests/fixtures/trace/minimal/trace.jsonl \
  --focus \
  --format dot
```

## Output Behavior

- If `--output` is provided, output is written to that file.
- If `--output` is omitted, output is printed to stdout.
