mod common;
use common::compare_template;

use ci_generate::{yarn::Yarn, CreateCi, TemplateData};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    let tmp_dir = temp_dir().join("yarn");
    let data = TemplateData::new(&tmp_dir)
        .license("MIT")
        .branch("main")
        .name("Yarn-project");

    Yarn::new()
        .create_ci(
            data
            /*data.get_name(),
            data.get_path(),
            data.get_license(),
            data.get_branch(),*/
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
