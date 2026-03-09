# Changelog - dustviz (DPL Execution Visualizer)

All notable changes to dustviz are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-03-09

### Changed

- Migrated `dustviz` from Rust crate layout to Dust-native workspace layout.
- Replaced `Cargo.toml` + `crates/dustviz` with `State.toml` + `src/*.ds` modules.
- Ported CLI/input/graph/render/app flow into Dust modules.
- Updated docs and CI to the Dust-native structure.
- Collapsed runtime build profile to compiler-compatible single-file top-level grammar (`src/main.ds`, `K main`).

## [0.2.0] - 2026-02-12 (DPL v0.2)

### Added

- **DPL v0.2 Compliance**: Full support for v0.2 specification
- Visualization for K Regime v0.2 execution
- Memory state visualization
- Variable state tracking
- Control flow graph visualization
- Call stack visualization
- Memory allocation/deallocation tracking
- Execution timeline view

### Changed

- Enhanced UI for complex program visualization
- Improved performance for large traces
- Better interactive controls

### Fixed

- Visualization of complex control flow
- Memory leak visualization accuracy

## [0.1.0] - 2026-02-12

### Added

- Initial visualization tool
- Basic execution trace viewing
- Simple emit output display

### Known Issues

- Limited visualization for v0.1 subset

---

Copyright © 2026 Dust LLC
