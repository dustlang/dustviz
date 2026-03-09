# Diagnostics

DustViz exposes deterministic status codes via `src/viz_contracts.ds`.

Primary statuses:

- `STATUS_OK`
- `ERR_USAGE`
- `ERR_INVALID_ARGUMENT`
- `ERR_MISSING_FLAG_VALUE`
- `ERR_MISSING_INPUT`
- `ERR_INVALID_FORMAT`
- `ERR_INPUT_RESOLUTION_FAILED`
- `ERR_INPUT_LOAD_FAILED`
- `ERR_GRAPH_BUILD_FAILED`
- `ERR_OVERLAY_FAILED`
- `ERR_RENDER_FAILED`
- `ERR_FOCUS_REQUIRES_TRACE`

`src/viz_host.ds` reports terminal outcome through host output.
