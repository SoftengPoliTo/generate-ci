mod common;
use common::compare_template;

use ci_generate::{poetry::Poetry, CommonData, CreateProject};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
fn test_poetry() {
    let tmp_dir = temp_dir().join("poetry");
    let common_data = CommonData::new()
        .license("MIT")
        .branch("main")
        .name("Poetry-project")
        .project_path(&tmp_dir);

    Poetry::new()
        .create_project(
            common_data.get_name(),
            common_data.get_path(),
            common_data.get_license(),
            common_data.get_branch(),
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
