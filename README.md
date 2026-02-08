# Dust

**Dust** is a phase-aware programming system centered around the **Dust Programming Language (DPL)**, its compiler toolchain, runtime, and associated analysis and visualization tools.

Dust is designed for **constraint-driven computation**, explicit **phase structure**, and formal reasoning about **program admissibility**, rather than treating execution as the sole arbiter of correctness.

This repository contains the full Dust toolchain, including the canonical DPL specification, compiler, runtime, and visualization infrastructure.

---

## Repository Structure

```
dust-main/
├── crates/
│   ├── dust_frontend/     # Lexer, parser, AST, surface syntax
│   ├── dust_semantics/    # Canonical semantic analysis & constraints
│   ├── dust_codegen/     # Lowering & code generation
│   ├── dust_driver/      # Compiler orchestration (dustc)
│   ├── dustrun/          # Runtime / execution harness
│   └── dustviz/          # IR + constraint graph visualization
│
├── spec/                 # ✅ CANONICAL DPL SPECIFICATION
│
├── examples/             # Example DPL programs
├── tests/                # Integration & conformance tests
├── Cargo.toml            # Workspace root
└── README.md             # (this file)
```

---

## Canonical vs Non-Canonical Material

### ✅ Canonical

The **only canonical definition of DPL** lives in:

```
spec/
```

Everything under `spec/` is **normative**:

- Language syntax
- Semantic rules
- Constraint definitions
- Phase behavior
- Admissibility rules
- Formal guarantees

If there is *any discrepancy* between code, comments, examples, documentation, or tooling **and the spec**, **the spec wins**.

---

### ⚠️ Non-Canonical (May Be Correct)

Everything outside `spec/`:

- Rust crates
- Comments
- Examples
- Tests
- Tooling behavior
- Documentation (including this file)

**may be correct**, but is **not authoritative**.

Non-canonical material is expected to:
- Lag the spec
- Explore design space
- Contain experimental or provisional behavior
- Be refined or replaced as the spec evolves

---

## Dust Programming Language (DPL)

DPL is not a conventional imperative or functional language.

Core characteristics:

- **Phase-aware semantics**
- **Constraint-first reasoning**
- **Admissibility over execution**
- **Explicit semantic structure**
- **Non-accidental complexity elimination**

A DPL program is not merely *run* — it is **validated**, **constrained**, and only then **admitted** for execution.

---

## Toolchain Overview

### `dustc` (Compiler)

The Dust compiler performs:

1. Parsing and AST construction
2. Canonical semantic analysis
3. Constraint generation
4. Phase validation
5. IR lowering
6. Code generation

The compiler’s behavior is governed by the canonical rules in `spec/`.

---

### `dustrun` (Runtime)

`dustrun` executes **admitted Dust programs**.

It is intentionally separated from compilation to preserve:
- Semantic clarity
- Determinism
- Inspectability
- Runtime minimalism

Execution does **not** redefine correctness.

---

### `dustviz` (Visualization)

`dustviz` is a first-class analysis tool that visualizes:

- Intermediate Representation (IR)
- Constraint graphs
- Phase dependencies
- Admissibility structure

Its purpose is **explanation, inspection, and verification**, not compilation or execution.

Outputs may include:
- Graphviz DOT
- SVG
- JSON (machine-readable)

---

## Design Philosophy

Dust is built around several core principles:

- **Semantics before mechanics**
- **Constraints before control flow**
- **Structure before optimization**
- **Clarity before convenience**
- **Admissibility before execution**

Dust is intentionally opinionated.

---

## Status

- The DPL specification is **actively evolving**
- The compiler and tools are **real, operational, and incomplete by design**
- Backward compatibility is **not guaranteed** outside the spec

Expect rapid iteration.

---

## License

See the repository license files for terms covering:
- Code
- Specification
- Documentation

---

## Contributing

Contributions are welcome **only if they respect the canonical authority of the spec**.

When in doubt:
1. Check `spec/`
2. Update `spec/`
3. Then update code

Never the reverse.