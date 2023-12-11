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
            common_data.get_name(),
            common_data.get_path(),
            common_data.get_license(),
            common_data.get_branch(),
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
