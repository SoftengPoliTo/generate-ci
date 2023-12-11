mod common;
use common::compare_template;

use ci_generate::{yarn::Yarn, CommonData, CreateCi};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    let tmp_dir = temp_dir().join("yarn");
    let yarn_data = CommonData::new("MIT", "main", "Yarn-project", &tmp_dir);

    Yarn::new()
        .create_ci(
            yarn_data.name,
            yarn_data.project_path,
            yarn_data.license,
            yarn_data.branch,
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
