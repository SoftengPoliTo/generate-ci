mod common;
use common::compare_template;

use ci_generate::{cargo::Cargo, CommonData, CreateCi};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/cargo/";

#[test]
fn test_cargo() {
    let tmp_dir = temp_dir().join("cargo");
    let cargo_data = Cargo::new().docker_image_description("description-docker");
    let common_data = CommonData::new()
        .license("MIT")
        .branch("master")
        .project_path(&tmp_dir);

    cargo_data
        .create_ci(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
