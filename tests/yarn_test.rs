mod common;
use common::compare_template;

use ci_generate::{yarn::Yarn, CreateCi, TemplateData};
use std::env::temp_dir;
use std::fs::create_dir_all;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    let tmp_dir = temp_dir();
    create_dir_all(tmp_dir.join("yarn")).unwrap();
    let path = tmp_dir.join("yarn");
    let data = TemplateData::new(&path)
        .license("MIT")
        .branch("main")
        .name("Yarn-project");

    Yarn::new().create_ci(data).unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &path);
}
