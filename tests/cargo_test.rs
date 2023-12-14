mod common;
use common::compare_template;

use ci_generate::{cargo::Cargo, CreateCi, TemplateData};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH_B: &str = "../repositories/snapshots/cargo/";
//const SNAPSHOT_PATH_L: &str = "../repositories/snapshots/cargo_library/";
//const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/cargo_ci/";

#[test]
fn test_cargo_binary() {
    let tmp_dir = temp_dir().join("cargo");
    let data = TemplateData::new(&tmp_dir).license("MIT").branch("master");

    Cargo::new()
        .docker_image_description("description-docker")
        .create_ci(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_B), &tmp_dir);
}
/*
#[test]
fn test_cargo_library() {
    let tmp_dir = temp_dir().join("cargo_library");
    let data = TemplateData::new(&tmp_dir).license("MIT").branch("main");

    Cargo::new()
        .docker_image_description("description-docker")
        .create_lib()
        .create_ci(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_L), &tmp_dir);
}
#[test]
fn test_cargo_ci() {
    let tmp_dir = temp_dir().join("cargo_onlyCi");
    let data = TemplateData::new(&tmp_dir).license("MIT").branch("main");

    Cargo::new()
        .docker_image_description("description-docker")
        .only_ci()
        .create_ci(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_C), &tmp_dir);
}
*/
