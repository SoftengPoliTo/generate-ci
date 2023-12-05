mod common;
use common::*;

use ci_generate::error::Result;
use std::{fs, path::Path};

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
fn test_poetry() {
    fs::create_dir(REPO).unwrap();
    create_poetry_project().unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}

fn create_poetry_project() -> Result<()> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::poetry::Poetry::new(),
        "",
        Path::new(REPO),
        "BSD-3-Clause",
        "main",
    )
}
