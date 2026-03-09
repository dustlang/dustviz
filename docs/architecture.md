# Architecture

DustViz remains a strict staged pipeline:

`CLI -> Input -> Graph -> Render`

Dust modules:

- `src/main.ds`: executable entrypoint.
- `src/viz_cli.ds`: host CLI parsing + command dispatch.
- `src/viz_app.ds`: orchestration for parse/render flows.
- `src/viz_input.ds`: input resolution + artifact loading.
- `src/viz_graph.ds`: graph build, overlays, focus transforms.
- `src/viz_render.ds`: DOT/JSON/SVG renderers.
- `src/viz_contracts.ds`: statuses, errors, command/format IDs.
- `src/viz_host.ds`: host bridge wrappers.
