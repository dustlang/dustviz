# Graph and Overlays

Graph flow is modeled by deterministic graph tokens:

1. `build_dir_graph(program_token)`
2. optional `overlay_constraints(graph, constraints)`
3. optional `link_constraint_ir_refs(graph, constraints, map)`
4. optional `overlay_trace(graph, trace)`
5. optional `focus_graph(graph, focus_nodes)`

All graph operations are in `src/viz_graph.ds`.
