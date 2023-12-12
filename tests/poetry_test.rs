mod common;
use common::compare_template;

use ci_generate::{poetry::Poetry, CreateProject, TemplateData};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/poetry/";

#[test]
fn test_poetry() {
    let tmp_dir = temp_dir().join("poetry");
    let data = TemplateData::new(&tmp_dir)
        .license("MIT")
        .branch("main")
        .name("Poetry-project");

    Poetry::new()
        .create_project(
            data, /*data.get_name(),
                 data.get_path(),
                 data.get_license(),
                 data.get_branch(),*/
        )
        .unwrap();

    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
