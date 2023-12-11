mod common;
use common::compare_template;

use ci_generate::{
    cargo::{Cargo, CargoData},
    CommonData, CreateCi,
};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/cargo/";

#[test]
fn test_cargo() {
    let cargo_data = CargoData::new("description-docker");
    let common_data = CommonData::new("MIT", "master", "", Path::new(REPO));
    Cargo::new(cargo_data.docker_image_description)
        .create_ci(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
