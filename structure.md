dustviz/
├── State.toml
├── README.md
├── src/
│   ├── main.ds              # Executable entrypoint
│   ├── dustviz.ds           # Public facade
│   ├── viz_contracts.ds     # Status/error/format contracts
│   ├── viz_host.ds          # Host bridge wrappers
│   ├── viz_cli.ds           # CLI parsing + dispatch
│   ├── viz_input.ds         # Input resolution/loading
│   ├── viz_graph.ds         # Graph build + overlays + focus
│   ├── viz_render.ds        # DOT/JSON/SVG renderers
│   ├── viz_app.ds           # Pipeline orchestration
│   └── viz_tests.ds         # Deterministic self-tests
├── tests/
│   └── fixtures/
│       └── minimal/
└── docs/
    ├── architecture.md
    ├── cli_reference.md
    ├── diagnostics.md
    ├── getting_started.md
    ├── graph_and_overlays.md
    ├── input_formats.md
    ├── library_api.md
    ├── rendering.md
    └── testing.md
