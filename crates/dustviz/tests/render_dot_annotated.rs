// crates/dustviz/tests/render_dot_annotated.rs
//
// Snapshot-style test for annotated DOT rendering.
//
// This test locks the annotated DOT output for the minimal fixture. If the output changes,
// it should be a deliberate change accompanied by updating the golden file.
//
// Golden file location:
// - tests/fixtures/minimal/program.annotated.dot

use std::fs;
use std::path::PathBuf;

use dustviz::graph::build_dir_graph;
use dustviz::input::load_dir_program;
use dustviz::render::render_dot_annotated;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("minimal")
}

fn program_path() -> PathBuf {
    fixture_dir().join("program.dir.json")
}

fn golden_dot_path() -> PathBuf {
    fixture_dir().join("program.annotated.dot")
}

#[test]
fn annotated_dot_output_matches_golden() {
    let program = load_dir_program(&program_path()).expect("fixture parses");
    let graph = build_dir_graph(&program);

    let actual = render_dot_annotated(&graph);
    let expected =
        fs::read_to_string(&golden_dot_path()).expect("golden annotated DOT file is present");

    let actual = actual.replace("\r\n", "\n");
    let expected = expected.replace("\r\n", "\n");

    assert_eq!(
        expected, actual,
        "annotated DOT output differs from golden file: {}",
        golden_dot_path().display()
    );
}