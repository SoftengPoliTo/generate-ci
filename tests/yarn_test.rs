mod common;
use common::compare_template;

use ci_generate::{yarn::Yarn, CommonData, CreateCi};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    let yarn_data = CommonData::new("MIT", "main", "Yarn-project", Path::new(REPO));

    Yarn::new()
        .create_ci(
            yarn_data.name,
            yarn_data.project_path,
            yarn_data.license,
            yarn_data.branch,
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
