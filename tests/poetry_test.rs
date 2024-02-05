mod common;

use std::env::temp_dir;
use std::path::Path;

use generate_ci::{poetry::Poetry, CreateProject, TemplateData};

use common::compare_template;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
fn test_poetry() {
    let tmp_dir = temp_dir();
    let path = tmp_dir.join("poetry");
    let data = TemplateData::new(&path, "poetry-python")
        .license("MIT")
        .branch("main");

    Poetry::new().create_project(data).unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &path);
}
