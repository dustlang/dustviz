// File: mod.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Input module - loading DIR artifacts and traces.
//   Submodules: discover, load, load_constraints, load_trace.

pub mod discover;
pub mod load;
pub mod load_constraints;
pub mod load_trace;

pub use discover::resolve_input_path;
pub use load::load_dir_program;
pub use load_constraints::load_constraints;
pub use load_trace::load_trace;
