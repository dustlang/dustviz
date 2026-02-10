// crates/dustviz/tests/render_trace_focus_dot.rs
//
// Snapshot-style test for focus DOT rendering with trace overlay.

use std::fs;
use std::path::PathBuf;

use dustviz::graph::{build_dir_graph, focus_graph, overlay_trace};
use dustviz::input::{load_dir_program, load_trace};
use dustviz::render::render_dot;

fn program_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("minimal")
        .join("program.dir.json")
}

fn trace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("trace")
        .join("minimal")
        .join("trace.jsonl")
}

fn golden_dot_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("trace")
        .join("minimal")
        .join("combined.focus.dot")
}

#[test]
fn dot_focus_with_trace_matches_golden() {
    let program = load_dir_program(&program_path()).expect("program fixture parses");
    let mut graph = build_dir_graph(&program);

    let trace = load_trace(&trace_path()).expect("trace fixture parses");
    let overlay = overlay_trace(&mut graph, &trace);

    let focused = focus_graph(&graph, &overlay.focus_nodes);

    let actual = render_dot(&focused);
    let expected = fs::read_to_string(&golden_dot_path()).expect("golden DOT is present");

    let actual = actual.replace("\r\n", "\n");
    let expected = expected.replace("\r\n", "\n");

    assert_eq!(
        expected, actual,
        "DOT output differs from golden file: {}",
        golden_dot_path().display()
    );
}
