# Testing

Self-tests live in `src/viz_tests.ds`.

Current deterministic checks:

- parse success path
- parse missing-input failure path
- render focus-without-trace failure path
- render JSON success path

Run via facade:

- `DustViz::K::run_self_tests()`
