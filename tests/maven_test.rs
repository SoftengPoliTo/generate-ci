mod common;

use common::compare_template_output_with_expected_one;

const REPO: &str = "tests/repositories/maven_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

// $ ci-generate maven --license=BSD-1-Clause --name=Myprog --branch=master POL tests/repositories/maven_template
#[test]
fn test_maven() {
    compare_template_output_with_expected_one(SNAPSHOT_PATH, REPO);
}