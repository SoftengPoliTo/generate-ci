mod common;

use std::path::PathBuf;

use common::*;

const REPO: &str = "tests/repositories/yarn_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

// $ ci-generate yarn --license=Apache-2.0 --name=Test_Project --branch=main tests/repositories/yarn_template
#[test]
fn test_yarn() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}
