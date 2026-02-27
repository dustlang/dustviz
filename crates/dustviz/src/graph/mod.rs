// File: mod.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Graph module - graph building and transformation.
//   Submodules: build, focus, link_ir_refs, overlay_constraints, overlay_trace.

pub mod build;
pub mod focus;
pub mod link_ir_refs;
pub mod overlay_constraints;
pub mod overlay_trace;

pub use build::{build_dir_graph, Attr, Edge, EdgeId, EdgeKind, Graph, Node, NodeId, NodeKind};
pub use focus::focus_graph;
pub use link_ir_refs::link_constraint_ir_refs;
pub use overlay_constraints::overlay_constraints;
pub use overlay_trace::{overlay_trace, TraceOverlay};
