# dustviz

`dustviz` is the **IR and constraint visualization tool** for the Dust Programming Language (DPL) toolchain.

It consumes compiler- and runtime-produced artifacts and renders **explicit graph representations** of:

- Intermediate Representation (IR)
- Semantic constraints
- Phase dependencies
- Admissibility structure

`dustviz` is an *analysis and inspection tool*.  
It does **not** compile, execute, or validate programs.

---

## Scope

`dustviz` is responsible for **making internal structure visible**.

Specifically, it:

- Loads IR and constraint data produced by the Dust compiler or runtime
- Builds explicit graph models from that data
- Emits deterministic visual or machine-readable representations

It does **not**:

- Define semantics
- Enforce admissibility
- Perform compilation
- Perform execution

All semantic meaning originates in the **canonical DPL specification**.

---

## Canonical Authority

The canonical definition of DPL semantics, constraints, and phases lives in:

```
spec/
```

`dustviz` **must not reinterpret, redefine, or extend** the spec.

If a visualization appears to contradict the spec, the visualization is wrong.

---

## Inputs

`dustviz` operates on **explicit artifacts**, not source code.

Initial supported inputs are:

- Compiler IR dumps
- Constraint sets emitted by semantic analysis
- Runtime execution traces (where applicable)

The exact input formats are intentionally isolated behind a loader layer so they can evolve without destabilizing visualization logic.

---

## Outputs

`dustviz` produces **pure representations** of structure.

Supported output formats:

- **DOT** (Graphviz)
- **SVG** (via DOT or native rendering)
- **JSON** (machine-readable graph model)

All outputs are deterministic given identical inputs.

---

## Architecture

High-level internal structure:

```
cli → input → model → graph → render
```

Where:

- `cli` parses user intent
- `input` loads artifacts
- `model` represents IR and constraints
- `graph` constructs structural relationships
- `render` emits outputs

Each stage is isolated and testable.

---

## Command-Line Interface

Example (illustrative):

```
dustviz render \
  --input target/dust/ir.json \
  --constraints target/dust/constraints.json \
  --format dot \
  --output graph.dot
```

The CLI is intentionally explicit; there is no “magic discovery”.

---

## Determinism

`dustviz` is designed to be:

- Deterministic
- Side-effect free
- Read-only with respect to inputs

Given the same inputs, it must produce the same outputs byte-for-byte.

---

## Status

`dustviz` is under active development.

Expect:
- Format changes
- Expanded graph semantics
- Additional renderers

Do **not** assume stability outside of what is explicitly documented here.

---

## Relationship to Other Tools

- `dustc` produces data
- `dustrun` executes admitted programs
- `dustviz` explains structure

Each tool has a single responsibility.

---

## License

See the repository license for terms.