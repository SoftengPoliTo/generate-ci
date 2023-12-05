mod common;
use common::*;

use ci_generate::error::Result;
use std::{fs, path::Path};

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

#[test]
fn test_maven() {
    fs::create_dir(REPO).unwrap();
    create_maven_project().unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}

fn create_maven_project() -> Result<()> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::maven::Maven::new("GRO"),
        "",
        Path::new(REPO),
        "BSD-1-Clause",
        "main",
    )
}
