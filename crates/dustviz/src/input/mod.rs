// crates/dustviz/src/input/mod.rs

pub mod discover;
pub mod load;

pub use discover::resolve_input_path;
pub use load::load_dir_program;