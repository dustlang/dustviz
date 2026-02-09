// crates/dustviz/src/render/mod.rs

pub mod dot;
pub mod json;

pub use dot::{render_dot, render_dot_annotated};
pub use json::render_json;