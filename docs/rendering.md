# Rendering

Renderer module: `src/viz_render.ds`

Supported formats:

- DOT
- JSON
- SVG

Render dispatch:

- `render_dot`
- `render_dot_annotated`
- `render_json`
- `render_svg`

Output is sent through host output handling in `src/viz_host.ds`.
