This directory contains minimal, deterministic test fixtures for dustviz.

The files here are used only for testing dustviz itself:
- JSON parsing
- CLI smoke tests
- Graph construction tests (added later)

They are not canonical, authoritative, or normative.
They exist solely to support development and CI.

program.dir.json
----------------
A minimal DIR Program JSON artifact that exercises:
- forge loading
- procedure loading
- statement enum deserialization

If this file changes, corresponding tests must be updated.