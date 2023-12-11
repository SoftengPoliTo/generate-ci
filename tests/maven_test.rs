mod common;
use common::compare_template;

use ci_generate::{maven::Maven, CommonData, CreateProject};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

#[test]
fn test_maven() {
    let tmp_dir = temp_dir().join("maven");
    let maven_data = Maven::new().group("POL");
    let common_data = CommonData::new()
        .license("BSD-1-Clause")
        .branch("main")
        .name("Maven-project")
        .project_path(&tmp_dir);

    maven_data
        .create_project(
            common_data.get_name(),
            common_data.get_path(),
            common_data.get_license(),
            common_data.get_branch(),
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
