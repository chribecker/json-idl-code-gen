// Integration tests for main.rs logic
// These tests use the binary crate and check for correct output and error handling.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use similar_asserts::assert_eq;
use std::env;

// boolean constant to enable/disable expected file overwrite
const OVERWRITE_EXPECTED: bool = false;

#[test]
fn test_main_generates_files() {
    let bin = env::var("COMGEN_BIN").expect("COMGEN_BIN not set");
    eprint!("Running test_main_generates_files {}\n", bin);
    let output_dir = "test_output";
    let _ = fs::remove_dir_all(output_dir); // Clean up before test
    let mut cmd = Command::new(bin);
    cmd.arg("-i")
        .arg("tests/car_window_types.yaml")
        .arg("-o")
        .arg(output_dir);
    cmd.assert().success();
    // Check that output files are created (example: car_window_types.h, car_window_types.rs, etc.)
    let expected_files = [
        "car_window_types.h",
        "car_window_types.rs",
        "car_window_types.cpp",
        "car_window_types_skeleton.cpp",
    ];
    for file in expected_files.iter() {
        let path = Path::new(output_dir).join(file);
        assert!(
            path.exists(),
            "Expected output file {:?} does not exist",
            path
        );
        let  gen_content = fs::read_to_string(&path).expect("Failed to read generated file");
        // compare content with the files in expected folder
        let epath = Path::new("tests/expected").join(file);
        // if expected file does not exist, create it with the generated content
        if OVERWRITE_EXPECTED || !epath.exists() {
            fs::write(&epath, &gen_content).expect("Failed to create expected file");
        }
        assert!(
            epath.exists(),
            "Expected output file {:?} does not exist",
            epath
        );
        let  expected_content = fs::read_to_string(&epath).expect("Failed to read expected file");
        assert_eq!(gen_content, expected_content, "Content mismatch in file {:?}", file);
    }
    let _ = fs::remove_dir_all(output_dir); // Clean up after test
}

#[test]
fn test_main_fails_on_missing_input() {
    let bin = env::var("COMGEN_BIN").expect("COMGEN_BIN not set");
    let mut cmd = Command::new(bin);
    cmd.arg("-i")
        .arg("test/does_not_exist.yaml")
        .arg("-o")
        .arg("test_output");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read input file"));
}
