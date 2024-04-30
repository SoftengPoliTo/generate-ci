mod common;

use std::env::temp_dir;
use std::path::Path;

use generate_ci::{yarn::Yarn, CreateCi, TemplateData};

use common::compare_template;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

#[test]
fn test_yarn() {
    let tmp_dir = temp_dir();
    let path = tmp_dir.join("yarn");
    let data = TemplateData::new(&path, "yarn-javascript", "SoftengPoliTo")
        .license("MIT")
        .branch("main");

    Yarn::new().create_ci(data).unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &path);
}
