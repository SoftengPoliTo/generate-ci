mod common;

use std::path::PathBuf;

use common::compare_template_output_with_expected_one;

const REPO: &str = "tests/repositories/cargo_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/cargo/";

// $ ci-generate cargo --docker-image-description=docker_image --license=EUPL-1.2 --name=Project --branch=main tests/repositories/cargo_template
#[test]
fn test_cargo() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}