mod common;

use anyhow::Error;

use common::compare_template_output_with_expected_one;

const REPO: &str = "tests/repositories/poetry_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

// $ ci-generate poetry --license=BSD-1-Clause --name=Myprog --branch=master tests/repositories/snapshots/poetry_project
pub fn create_poetry_template() -> Result<(), Error> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::poetry::Poetry::new(),
        "Myprog",
        std::path::Path::new("./tests/repositories/poetry_template"),
        "BSD-1-Clause",
        "master",
    )
}

#[test]
fn test_poetry() {
    let _poetry = create_poetry_template().unwrap();
    compare_template_output_with_expected_one(SNAPSHOT_PATH, REPO);
}