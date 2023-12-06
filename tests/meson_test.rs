mod common;
use common::compare_template_output_with_expected_one;

use ci_generate::{meson::{Meson, ProjectKind}, CreateProject};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

#[test]

fn test_meson() {
    Meson::new(ProjectKind::C).create_project(
        "", 
        Path::new(REPO), 
        "BSD-3-Clause", 
        "main"
    ).unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
