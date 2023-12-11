mod common;
use common::compare_template;

use ci_generate::{yarn::Yarn, CommonData, CreateCi};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    let tmp_dir = temp_dir().join("yarn");
    let common_data = CommonData::new()
        .license("MIT")
        .branch("main")
        .name("Yarn-project")
        .project_path(&tmp_dir);

    Yarn::new()
        .create_ci(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
