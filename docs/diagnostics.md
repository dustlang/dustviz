# Diagnostics and Error Handling

Sources:

- `src/util/diagnostics.rs`
- usage sites across `src/main.rs`, `src/input/*`, `src/render/svg.rs`, `src/util/fs.rs`

## Diagnostic Enum

`Diagnostic` variants:

1. `Io { path, source }`
2. `Json { path, source }`
3. `Message(String)`

Helper:

- `Diagnostic::message(msg)` creates `Message`.

## Error Formatting

Formatted messages are human-readable and include paths when available.

Examples:

- `I/O error while reading <path>: <source>`
- `failed to parse JSON in <path>: <source>`
- custom messages from `Message`

## Where Errors Are Produced

### Filesystem and path

- `fs::canonicalize`
- `fs::read_to_string`
- output file writes in `main.rs::write_output`

### JSON parse

- DIR parse (`load_dir_program`)
- constraints parse (`load_constraints`)
- trace line parse (`load_trace`, message includes line number)

### Runtime/CLI logic

- missing input path (`resolve_input_path`)
- invalid focus usage (`--focus` without `--trace`)
- SVG renderer process failures (`render_svg`)

## CLI Failure Behavior

In `main.rs`:

- diagnostics are printed as `error: <diagnostic>`
- process exits with status code `1`
