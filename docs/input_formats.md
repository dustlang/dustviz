# Input Formats

DustViz accepts external artifact paths through CLI flags:

- `--input` (required)
- `--constraints` (optional)
- `--trace` (optional)

In the Dust-native implementation, loaders in `src/viz_input.ds` resolve and normalize inputs into deterministic artifact tokens used by later stages.
