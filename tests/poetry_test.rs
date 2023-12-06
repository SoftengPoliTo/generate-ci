mod common;
use common::compare_template_output_with_expected_one;

use ci_generate::{poetry::Poetry, CommonData, CreateProject};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
fn test_poetry() {
    let poetry_data = CommonData::new("MIT", "main", "Poetry-project", Path::new(REPO));
    Poetry::new()
        .create_project(
            poetry_data.name,
            poetry_data.project_path,
            poetry_data.license,
            poetry_data.branch,
        )
        .unwrap();

    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
