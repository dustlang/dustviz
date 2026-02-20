# Rendering

Sources:

- `src/render/dot.rs`
- `src/render/json.rs`
- `src/render/svg.rs`

## DOT Renderer

Functions:

- `render_dot(graph) -> String`
- `render_dot_annotated(graph) -> String`

`render_dot` emits stable plain DOT output.
`render_dot_annotated` keeps labels unchanged and surfaces attrs as Graphviz `tooltip` attributes.

### DOT Node Shape Mapping

- Program: `oval`
- Forge: `folder`
- Shape: `box3d`
- Proc: `component`
- Uses: `note`
- Bind: `diamond`
- Clause: `parallelogram`
- Stmt: `box`
- TraceEvent: `octagon`

### DOT Edge Labels

- contains
- next
- uses
- clause
- trace

### Escaping

DOT renderer escapes:

- backslash,
- double quote,
- newline (`\n`).

## JSON Renderer

Function:

- `render_json(graph) -> Result<String, serde_json::Error>`

Output schema:

```json
{
  "nodes": [
    { "id": 0, "kind": "Program", "label": "Program", "attrs": {} }
  ],
  "edges": [
    { "id": 0, "kind": "contains", "from": 0, "to": 1, "attrs": {} }
  ]
}
```

Details:

- attrs are serialized via `BTreeMap` for deterministic key order,
- duplicate attr keys collapse to last value.

## SVG Renderer

Function:

- `render_svg(dot_text) -> Result<String, Diagnostic>`

Behavior:

1. spawns external process: `dot -Tsvg`
2. writes DOT input to stdin
3. reads SVG from stdout
4. returns diagnostic on spawn/run/failure/invalid UTF-8

If Graphviz is missing, renderer returns:

`Graphviz 'dot' not found in PATH; install Graphviz to enable SVG output.`

## CLI Format Selection

From `main.rs`:

- `dot`: render plain or annotated DOT directly
- `json`: render JSON graph
- `svg`: render DOT (plain or annotated), then convert with Graphviz
