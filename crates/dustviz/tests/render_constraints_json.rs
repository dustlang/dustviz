// crates/dustviz/tests/render_constraints_json.rs
//
// Snapshot-style test for JSON rendering with constraint overlay + IR-reference linking.
//
// This test locks the JSON output for the combined graph:
// - IR fixture: tests/fixtures/minimal/program.dir.json
// - Constraints fixture: tests/fixtures/constraints/minimal/constraints.json
//
// Golden file location:
// - tests/fixtures/constraints/minimal/combined.linked.graph.json

use std::fs;
use std::path::PathBuf;

use dustviz::graph::{build_dir_graph, link_constraint_ir_refs, overlay_constraints};
use dustviz::input::{load_constraints, load_dir_program};
use dustviz::render::render_json;

fn program_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("minimal")
        .join("program.dir.json")
}

fn constraints_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("constraints")
        .join("minimal")
        .join("constraints.json")
}

fn golden_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("constraints")
        .join("minimal")
        .join("combined.linked.graph.json")
}

#[test]
fn json_with_constraints_and_links_matches_golden() {
    let program = load_dir_program(&program_path()).expect("program fixture parses");
    let mut graph = build_dir_graph(&program);

    let constraints = load_constraints(&constraints_path()).expect("constraints fixture parses");
    let map = overlay_constraints(&mut graph, &constraints);
    link_constraint_ir_refs(&mut graph, &constraints, &map);

    let actual = render_json(&graph).expect("json renders");
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
