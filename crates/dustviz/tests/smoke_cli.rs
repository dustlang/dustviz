// File: smoke_cli.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   End-to-end smoke tests for dustviz CLI.
//   Verifies binary launches, parse command works, failures reported.

use assert_cmd::prelude::*;
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

    cmd.arg("parse").arg("--input").arg(fixture_path());

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
