mod common;

use std::path::PathBuf;

use common::compare_template_output_with_expected_one;

const REPO: &str = "tests/repositories/poetry_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

// $ ci-generate poetry --license=BSD-1-Clause --name=Myprog --branch=master tests/repositories/snapshots/poetry_project
#[test]
fn test_poetry() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}