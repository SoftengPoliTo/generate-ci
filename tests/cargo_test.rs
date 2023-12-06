mod common;
use common::compare_template_output_with_expected_one;

use ci_generate::{cargo::Cargo, CreateCi};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/cargo/";

#[test]
fn test_cargo() {
    Cargo::new("docker_image_description").create_ci(
        "",
        Path::new(REPO),
        "BSD-3-Clause",
        "main",
    ).unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
