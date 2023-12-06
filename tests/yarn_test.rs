mod common;
use common::compare_template_output_with_expected_one;

use ci_generate::{yarn::Yarn, CreateCi};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
#[ignore]
fn test_yarn() {
    Yarn::new().create_ci(
        "", 
        Path::new(REPO), 
        "BSD-3-Clause", 
        "main"
    ).unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
