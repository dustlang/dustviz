# Testing and Fixtures

Test root: `crates/dustviz/tests`

## Test Categories

1. CLI smoke tests
2. rendering snapshot tests
3. overlay snapshot tests
4. focus rendering snapshot test
5. graph builder unit tests (in `src/graph/build.rs`)

## CLI Smoke

File: `smoke_cli.rs`

- `parse_valid_dir_program_succeeds`
- `parse_missing_file_fails`

## Snapshot Tests

DOT/JSON snapshots compare generated outputs against fixture golden files.

### Base graph snapshots

- `render_dot.rs` / `render_json.rs`
- fixture: `tests/fixtures/minimal`

### Annotated DOT

- `render_dot_annotated.rs`
- golden: `program.annotated.dot`

### Constraints overlay snapshots

- `render_constraints_dot.rs`
- `render_constraints_json.rs`
- fixture: `tests/fixtures/constraints/minimal`

### Trace overlay snapshots

- `render_trace_dot.rs`
- `render_trace_json.rs`
- fixture: `tests/fixtures/trace/minimal`

### Focus mode snapshot

- `render_trace_focus_dot.rs`
- golden: `combined.focus.dot`

## Fixture Directories

- `tests/fixtures/minimal`
- `tests/fixtures/constraints/minimal`
- `tests/fixtures/trace/minimal`

## Snapshot Update Workflow

When behavior changes intentionally:

1. regenerate outputs with current code,
2. replace corresponding golden fixture files,
3. run full tests,
4. review diffs to ensure change is intentional.

## CI

Workflow file: `.github/workflows/ci.yml`

CI runs:

```bash
cargo test --workspace --verbose
```
