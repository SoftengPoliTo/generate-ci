mod common;
use common::compare_template;

use ci_generate::{cargo::Cargo, CreateCi, TemplateData};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/cargo/";

#[test]
fn test_cargo() {
    let tmp_dir = temp_dir().join("cargo");
    let data = TemplateData::new(&tmp_dir).license("MIT").branch("master");

    Cargo::new()
        .docker_image_description("description-docker")
        .create_ci(
            data.get_name(),
            data.get_path(),
            data.get_license(),
            data.get_branch(),
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
