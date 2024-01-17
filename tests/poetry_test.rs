mod common;
use common::compare_template;

use ci_generate::{poetry::Poetry, CreateProject, TemplateData};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
fn test_poetry() {
    let tmp_dir = temp_dir();
    let path = tmp_dir.join("poetry");
    let data = TemplateData::new(&path)
        .license("MIT")
        .branch("main")
        .name("Poetry-project");

    Poetry::new().create_project(data).unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &path);
}
