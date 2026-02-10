// crates/dustviz/tests/render_trace_json.rs
//
// Snapshot-style test for JSON rendering with trace overlay.

use std::fs;
use std::path::PathBuf;

use dustviz::graph::{build_dir_graph, overlay_trace};
use dustviz::input::{load_dir_program, load_trace};
use dustviz::render::render_json;

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

fn golden_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("trace")
        .join("minimal")
        .join("combined.graph.json")
}

#[test]
fn json_with_trace_matches_golden() {
    let program = load_dir_program(&program_path()).expect("program fixture parses");
    let mut graph = build_dir_graph(&program);

    let trace = load_trace(&trace_path()).expect("trace fixture parses");
    let _overlay = overlay_trace(&mut graph, &trace);

    let actual = render_json(&graph).expect("json render");
    let expected = fs::read_to_string(&golden_json_path()).expect("golden JSON is present");

    let actual = actual.replace("\r\n", "\n");
    let expected = expected.replace("\r\n", "\n");
    let actual = actual.trim_end_matches('\n');
    let expected = expected.trim_end_matches('\n');

    assert_eq!(
        expected, actual,
        "JSON output differs from golden file: {}",
        golden_json_path().display()
    );
}
