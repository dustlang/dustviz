This directory contains minimal, deterministic test fixtures for dustviz.

Files here are used exclusively for testing:
- JSON parsing
- CLI smoke tests
- Graph construction tests (later)

They are not authoritative, canonical, or normative.
They are intentionally small and stable.

program.dir.json
----------------
A minimal DIR Program JSON artifact that exercises:
- forge loading
- procedure loading
- statement enum deserialization

If this file changes, corresponding tests must be updated.