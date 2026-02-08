dustviz/
├── Cargo.toml
├── README.md            # ROOT README (dustviz only)
│
├── src/
│   ├── main.rs          # dustviz binary entrypoint
│   ├── cli.rs           # CLI definition
│   ├── app.rs           # Orchestration layer
│   │
│   ├── input/           # External artifact loading
│   │   ├── mod.rs
│   │   ├── load.rs
│   │   └── discover.rs
│   │
│   ├── model/           # Internal data models
│   │   ├── mod.rs
│   │   ├── ir.rs
│   │   └── constraints.rs
│   │
│   ├── graph/           # Graph construction & analysis
│   │   ├── mod.rs
│   │   ├── build.rs
│   │   ├── annotate.rs
│   │   └── simplify.rs
│   │
│   ├── render/          # Output backends
│   │   ├── mod.rs
│   │   ├── dot.rs
│   │   ├── svg.rs
│   │   └── json.rs
│   │
│   └── util/
│       ├── mod.rs
│       ├── diagnostics.rs
│       └── fs.rs
│
└── tests/
    ├── smoke_cli.rs
    └── fixtures/
        └── minimal/