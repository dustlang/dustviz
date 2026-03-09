# CLI Reference

Commands:

- `parse --input <dir.json>`
- `render --input <dir.json> [--constraints <constraints.json>] [--trace <trace.jsonl>] [--format dot|json|svg] [--annotated] [--focus] [--output <path>]`

Behavior notes:

- `--format` defaults to `dot`.
- `--focus` requires `--trace`.
- unknown flags return `ERR_INVALID_ARGUMENT`.
- missing values return `ERR_MISSING_FLAG_VALUE`.
