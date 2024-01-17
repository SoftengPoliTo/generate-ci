mod common;
use common::compare_template_skip;

use ci_generate::{cargo::Cargo, CreateCi, TemplateData};
use std::env::temp_dir;
use std::fs::create_dir_all;
use std::path::Path;

const SKIPPED_FOLDERS: &[&str] = &[".git", "Cargo.lock"];
const SNAPSHOT_PATH_B: &str = "../repositories/snapshots/cargo/";
const SNAPSHOT_PATH_L: &str = "../repositories/snapshots/cargo_library/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/cargo_ci/";

#[test]
fn test_cargo_binary() {
    let tmp_dir = temp_dir();
    create_dir_all(tmp_dir.join("cargo")).unwrap();
    let path = tmp_dir.join("cargo");
    let data = TemplateData::new(&path).license("MIT").branch("master");

    Cargo::new()
        .docker_image_description("description-docker")
        .create_ci(data)
        .unwrap();
    compare_template_skip(Path::new(SNAPSHOT_PATH_B), &path, SKIPPED_FOLDERS);
}

#[test]
fn test_cargo_library() {
    let tmp_dir = temp_dir();
    create_dir_all(tmp_dir.join("cargo_library")).unwrap();
    let path = tmp_dir.join("cargo_library");
    let data = TemplateData::new(&path).license("MIT").branch("main");

    Cargo::new()
        .docker_image_description("description-docker")
        .create_lib()
        .create_ci(data)
        .unwrap();
    compare_template_skip(Path::new(SNAPSHOT_PATH_L), &path, SKIPPED_FOLDERS);
}
#[test]
fn test_cargo_ci() {
    let tmp_dir = temp_dir();
    create_dir_all(tmp_dir.join("cargo_ci")).unwrap();
    let path = tmp_dir.join("cargo_ci");
    let data = TemplateData::new(&path).license("MIT").branch("main");

    Cargo::new()
        .docker_image_description("description-docker")
        .only_ci()
        .create_ci(data)
        .unwrap();
    compare_template_skip(Path::new(SNAPSHOT_PATH_C), &path, SKIPPED_FOLDERS);
}
