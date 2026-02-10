// crates/dustviz/src/lib.rs
//
// Library entrypoint for dustviz.
//
// This allows unit tests and other crates in the workspace to reuse:
// - input loading
// - DIR model types
// - graph building
//
// The `dustviz` binary (`src/main.rs`) remains the CLI entrypoint.

pub mod app;
pub mod cli;
pub mod graph;
pub mod input;
pub mod model;
pub mod render;
pub mod util;
