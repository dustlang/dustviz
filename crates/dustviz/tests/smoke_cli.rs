// crates/dustviz/tests/smoke_cli.rs
//
// End-to-end smoke tests for the dustviz CLI.
//
// These tests verify that:
// - the binary launches
// - the `parse` command accepts a valid DIR artifact
// - failures are reported with a non-zero exit code
//
// No graph or rendering behavior is tested here.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("minimal")
        .join("program.dir.json")
}

#[test]
fn parse_valid_dir_program_succeeds() {
    let mut cmd = Command::cargo_bin("dustviz").expect("binary builds");

    cmd.arg("parse")
        .arg("--input")
        .arg(fixture_path());

    cmd.assert().success();
}

#[test]
fn parse_missing_file_fails() {
    let mut cmd = Command::cargo_bin("dustviz").expect("binary builds");

    cmd.arg("parse")
        .arg("--input")
        .arg("does_not_exist.dir.json");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}