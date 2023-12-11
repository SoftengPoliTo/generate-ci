mod common;
use common::compare_template;

use ci_generate::{poetry::Poetry, CommonData, CreateProject};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
#[ignore]
fn test_poetry() {
    let tmp_dir = temp_dir().join("poetry");
    let poetry_data = CommonData::new("MIT", "main", "Poetry-project", &tmp_dir);
    Poetry::new()
        .create_project(
            poetry_data.name,
            poetry_data.project_path,
            poetry_data.license,
            poetry_data.branch,
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
