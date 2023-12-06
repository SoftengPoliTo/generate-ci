mod common;
use common::compare_template_output_with_expected_one;

use ci_generate::{
    maven::{Maven, MavenData},
    CommonData, CreateProject,
};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

#[test]
fn test_maven() {
    let maven_data = MavenData::new("POL");
    let common_data = CommonData::new("BSD-1-Clause", "main", "Maven-project", Path::new(REPO));
    Maven::new(maven_data.group)
        .create_project(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
