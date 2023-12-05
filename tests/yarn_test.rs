mod common;
use common::*;

use ci_generate::error::Result;
use std::{fs, path::Path};

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    fs::create_dir(REPO).unwrap();
    create_yarn_project().unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}

fn create_yarn_project() -> Result<()> {
    ci_generate::CreateCi::create_ci(
        &ci_generate::toolchain::yarn::Yarn::new(),
        "",
        std::path::Path::new(REPO),
        "BSD-3-Clause",
        "main",
    )
}
