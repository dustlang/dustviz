// crates/dustviz/tests/render_json.rs
//
// Snapshot-style test for JSON rendering.
//
// This test locks the JSON output for the minimal fixture. If the output changes,
// it should be a deliberate change accompanied by updating the golden file.
//
// Golden file location:
// - tests/fixtures/minimal/program.graph.json

use std::fs;
use std::path::PathBuf;

use dustviz::graph::build_dir_graph;
use dustviz::input::load_dir_program;
use dustviz::render::render_json;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("minimal")
}

fn program_path() -> PathBuf {
    fixture_dir().join("program.dir.json")
}

fn golden_json_path() -> PathBuf {
    fixture_dir().join("program.graph.json")
}

#[test]
fn json_output_matches_golden() {
    let program = load_dir_program(&program_path()).expect("fixture parses");
    let graph = build_dir_graph(&program);

    let actual = render_json(&graph).expect("json renders");
    let expected =
        fs::read_to_string(&golden_json_path()).expect("golden JSON file is present");

    // Normalize Windows CRLF just in case (CI may vary).
    let actual = actual.replace("\r\n", "\n");
    let expected = expected.replace("\r\n", "\n");

    assert_eq!(
        expected, actual,
        "JSON output differs from golden file: {}",
        golden_json_path().display()
    );
}