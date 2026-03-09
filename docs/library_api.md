# Library API

`src/dustviz.ds` provides the public facade:

- `DustViz::K::parse(input_path)`
- `DustViz::K::render(input_path, constraints_path, trace_path, format, annotated, focus, output_path)`
- `DustViz::K::run_self_tests()`

`src/viz_app.ds` is the orchestration layer behind these facade calls.
