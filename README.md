# dustviz

`dustviz` is a visualization and inspection tool for **intermediate representations (IR)** and **constraint graphs** produced by external systems.

It is designed to make structural relationships explicit and inspectable by rendering them as graphs or machine-readable models.

`dustviz` does **not** define, validate, compile, or execute programs.

---

## Purpose

The purpose of `dustviz` is **structural visibility**.

It exists to:
- Load externally produced artifacts (such as IR or constraint data)
- Construct explicit graph representations from those artifacts
- Render those graphs in deterministic, inspectable formats

The tool is intended for:
- Debugging
- Analysis
- Explanation
- Verification by inspection

---

## Project Scope

This project is strictly limited to **visualization**.

### `dustviz` DOES:
- Consume external IR and constraint artifacts
- Build in-memory graph models
- Emit visual and machine-readable representations

### `dustviz` DOES NOT:
- Define language syntax or semantics
- Perform semantic validation
- Enforce correctness or admissibility
- Compile or execute code
- Own or embed any language specification

All inputs are treated as **opaque external data** whose meaning is defined elsewhere.

---

## Inputs

`dustviz` operates on **explicit input artifacts** supplied by the user.

Typical inputs include:
- Intermediate representation (IR) dumps
- Constraint sets
- Dependency or phase graphs
- Execution traces

Input formats are **not defined by this project**.
They are treated as external contracts and are isolated behind a loader layer so they can evolve independently.

---

## Outputs

`dustviz` produces **pure representations** of structure.

Supported output formats include:
- **DOT** (Graphviz)
- **SVG**
- **JSON** (machine-readable graph model)

All outputs are:
- Deterministic
- Side-effect free
- Derived solely from the provided inputs

---

## Architecture

High-level processing pipeline:

```
CLI → Input Loader → Internal Model → Graph Builder → Renderer
```

Each stage is isolated to ensure:
- Clear responsibility boundaries
- Replaceable input formats
- Multiple rendering backends
- Testability

---

## Command-Line Usage

Example (illustrative):

```
dustviz render \
  --input ir.json \
  --constraints constraints.json \
  --format dot \
  --output graph.dot
```

The CLI is intentionally explicit.
There is no implicit discovery or hidden behavior.

---

## Determinism

Given the same inputs and configuration, `dustviz` must produce the same outputs byte-for-byte.

This property is required for:
- Reliable debugging
- Regression testing
- Tooling integration

---

## Development Context

This project was developed using external artifacts and documentation provided alongside it for reference.

Those materials:
- Are not included in this project
- Are not owned by this project
- Are not documented here

They are treated strictly as **external references**.

---

## Status

`dustviz` is under active development.

Expect:
- New input adapters
- Expanded graph annotations
- Additional renderers

No stability guarantees are implied unless explicitly documented.

---

## License

See the repository license for terms.