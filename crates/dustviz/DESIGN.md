# dustviz — Design & Feature Plan (Operational, Not Aspirational)

`dustviz` is a visualization tool that consumes external artifacts (IR + constraints + traces) and emits deterministic graph representations (DOT/JSON/SVG). It does not compile, execute, or define semantics.

---

## 1. Core Goals

1. Deterministic, reproducible output (byte-for-byte snapshots).
2. Explicit pipeline boundaries: `Input → Model → Graph → Render`.
3. Multiple input formats via adapters without touching graph/render code.
4. Graph is the single internal “truth” for visualization.
5. Renderers are pure functions: `Graph → String/Bytes`.
6. Annotation-first: rich metadata exists even if not shown by default.

---

## 2. Primary Artifacts (Inputs)

### 2.1 DIR Program JSON (v0.1) ✅ implemented
- File: `*.dir.json` (test fixture: `tests/fixtures/minimal/program.dir.json`)
- Parsed into `model::ir`
- Converted into `graph::Graph`

### 2.2 Constraints JSON (v0.2)
Second input artifact, designed to overlay constraints on top of IR.

- File: `*.constraints.json`
- Contains:
  - constraint nodes (predicate, rule, obligation, witness, proof step)
  - edges (depends_on, justifies, blocks, requires, produces)
  - optional mapping keys to IR (proc name, stmt index, symbol id)

**Must support partial mapping**: constraints can exist without IR links.

### 2.3 Trace / Events (v0.3)
Optional runtime or compiler event stream.
- File: `*.trace.jsonl` (JSON lines for append-only streaming)
- Used to animate/highlight:
  - executed statements
  - constraint failures
  - solver iterations
  - rule firing order

---

## 3. Internal Models (Typed)

### 3.1 `model::ir`
- Minimal schema matching DIR JSON
- Opaque strings for expressions/types/predicates

### 3.2 `model::constraints`
- Constraint graph schema with stable IDs and optional IR references
- Example types:
  - ConstraintId
  - ConstraintNodeKind: Predicate | Rule | Obligation | Witness | Proof | Diagnostic
  - ConstraintEdgeKind: DependsOn | Justifies | Requires | Blocks | Produces | RefersTo

### 3.3 `model::trace`
- Event types with timestamps/order index
- Example:
  - Event::EnterProc(name)
  - Event::StmtExec(proc, index)
  - Event::ConstraintAdded(id)
  - Event::ConstraintFailed(id, reason)

---

## 4. Graph Layer (Single Unified Graph)

### 4.1 Node Kinds
- IR:
  - Program, Forge, Shape, Proc, Uses, Bind, Clause, Stmt
- Constraints overlay:
  - CNode (constraint node), CEdge (optional as explicit nodes for hyperedges), Diagnostic
- Trace overlay:
  - Event nodes (optional) OR event annotations

### 4.2 Edge Kinds
- Structural:
  - contains, next, uses, clause
- Constraint:
  - depends_on, justifies, requires, blocks, produces, refers_to
- Trace/Highlight:
  - occurred_after, highlights, triggered_by

### 4.3 Annotations (Attrs)
- Always stored on nodes/edges as key/value (deterministic ordering).
- Render visibility is controlled by options:
  - none (default)
  - tooltip-only (current “annotated dot”)
  - selected-visible keys
  - all-visible keys (debug)

**Key requirement**: adding annotations must not change default snapshots.

---

## 5. Rendering Backends

### 5.1 DOT
- Default, always available.
- Two modes:
  - `render_dot()` stable snapshots (no visible attrs)
  - `render_dot_annotated()` tooltips (attrs surfaced without label changes)

### 5.2 JSON Graph
- Machine-readable graph for web viewers and tooling.
- Stable schema:
  - nodes: id, kind, label, attrs
  - edges: id, kind, from, to, attrs

### 5.3 SVG
- Two tiers:
  1. DOT → SVG (invoke `dot -Tsvg`) if available
  2. Pure Rust renderer later if needed

SVG mode must remain deterministic given same `dot` version OR must embed the DOT and treat SVG as best-effort.

---

## 6. CLI Contract

### 6.1 Commands
- `parse --input <dir.json>` (verify load)
- `render --input <dir.json> [--constraints <constraints.json>] [--trace <trace.jsonl>]`
  - `--format dot|json|svg`
  - `--annotated` (tooltip surfacing)
  - `--output <path>` (else stdout)

### 6.2 Deterministic Behavior
- Sorting rules:
  - attrs sorted lexicographically
  - tooltips line ordering stable
  - graph node creation is traversal-ordered and stable

---

## 7. Testing Strategy (Hard Requirements)

1. Fixture parse tests (already).
2. Graph construction invariants (already).
3. Golden snapshot tests:
   - DOT (already)
   - JSON (already)
   - Annotated DOT (already)
4. Future:
   - constraint overlay snapshot test
   - “no regression” tests for node/edge counts under fixtures
   - fuzz-style “does not panic” tests for random JSON (optional)

---

## 8. Implementation Phases

### Phase A (v0.1) ✅
- DIR load
- Graph build
- DOT render + snapshots
- JSON render + snapshots
- Tooltips (annotated DOT) + snapshot

### Phase B (v0.2)
- Constraints artifact definition + loader
- Constraint overlay merge into graph
- Render snapshots for constraint fixture

### Phase C (v0.3)
- Trace artifact definition + loader
- Highlighting / annotation mapping
- Optional “focus” rendering flags (only nodes touched by trace)

### Phase D (v0.4)
- SVG pipeline (best-effort)
- Optional web viewer integration (out of Rust binary scope unless requested)

---

## 9. Non-Goals

- No semantic validation.
- No solver implementation.
- No compilation/execution.
- No canonical authority over external formats.

---