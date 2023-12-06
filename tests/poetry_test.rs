mod common;
use common::compare_template_output_with_expected_one;

use ci_generate::{poetry::Poetry, CreateProject};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
#[ignore]
fn test_poetry() {
    Poetry::new().create_project(
        "", 
        Path::new(REPO), 
        "", 
    "main"
    ).unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
