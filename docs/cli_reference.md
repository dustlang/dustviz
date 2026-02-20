# CLI Reference

Source: `crates/dustviz/src/cli.rs` and `crates/dustviz/src/main.rs`

## Command: parse

Validate and parse a DIR Program artifact.

### Required flags

- `--input <PATH>`: path to DIR JSON file.

### Behavior

- Loads and parses JSON into `DirProgram`.
- Builds graph to verify graph construction path.
- Produces no rendered output on success.

## Command: render

Render graph output from DIR input with optional overlays.

### Required flags

- `--input <PATH>`: path to DIR JSON file.

### Optional flags

- `--constraints <PATH>`: constraint JSON overlay file.
- `--trace <PATH>`: trace JSONL overlay file.
- `--format <dot|json|svg>`: output format. Default is `dot`.
- `--annotated`: include attribute tooltips in DOT/SVG path.
- `--focus`: focus graph to nodes touched by trace events.
- `--output <PATH>`: file output path (stdout when omitted).

### Important rule

`--focus` requires `--trace`.

If `--focus` is supplied without `--trace`, render fails with diagnostic:

`--focus requires a trace file via --trace`

## OutputFormat Enum

- `Dot`
- `Json`
- `Svg`

## Exit Behavior

- success: exit code `0`
- failure: prints `error: <message>` to stderr and exits `1`
