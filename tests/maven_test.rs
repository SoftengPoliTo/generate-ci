mod common;
use common::compare_template;

use ci_generate::{
    maven::{Maven, MavenData},
    CommonData, CreateProject,
};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

#[test]
fn test_maven() {
    let tmp_dir = temp_dir().join("maven");
    let maven_data = MavenData::new("POL");
    let common_data = CommonData::new("BSD-1-Clause", "main", "Maven-project", &tmp_dir);
    Maven::new(maven_data.group)
        .create_project(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
