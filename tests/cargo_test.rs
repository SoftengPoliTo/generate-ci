mod common;

use std::env::temp_dir;
use std::path::Path;

use generate_ci::{cargo::Cargo, CreateCi, TemplateData};

use common::compare_template_skip;

const SKIPPED_FOLDERS: &[&str] = &[".git", "Cargo.lock"];
const SNAPSHOT_PATH_B: &str = "../repositories/snapshots/cargo/";
const SNAPSHOT_PATH_L: &str = "../repositories/snapshots/cargo_library/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/cargo_ci/";

#[test]
fn test_cargo_binary() {
    // Rust nightly version can introduce changes making tests fail, so this
    // test is not executed on nightly
    if std::env::var("RUSTUP_TOOLCHAIN").map_or(true, |env| env.starts_with("nightly")) {
        return;
    }

    let tmp_dir = temp_dir();
    let path = tmp_dir.join("cargo");
    let data = TemplateData::new(&path, "cargo-rust-binary", "SoftengPoliTo")
        .license("MIT")
        .branch("master");

    Cargo::new()
        .docker_image_description("description-docker")
        .create_ci(data)
        .unwrap();
    compare_template_skip(Path::new(SNAPSHOT_PATH_B), &path, SKIPPED_FOLDERS);
}

#[test]
fn test_cargo_library() {
    // Rust nightly version can introduce changes making tests fail, so this
    // test is not executed on nightly
    if std::env::var("RUSTUP_TOOLCHAIN").map_or(true, |env| env.starts_with("nightly")) {
        return;
    }

    let tmp_dir = temp_dir();
    let path = tmp_dir.join("cargo_library");
    let data = TemplateData::new(&path, "cargo-rust-library", "SoftengPoliTo")
        .license("MIT")
        .branch("main");

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
    let path = tmp_dir.join("cargo_ci");
    let data = TemplateData::new(&path, "cargo-rust-ci", "SoftengPoliTo")
        .license("MIT")
        .branch("main");

    Cargo::new()
        .docker_image_description("description-docker")
        .only_ci()
        .create_ci(data)
        .unwrap();
    compare_template_skip(Path::new(SNAPSHOT_PATH_C), &path, SKIPPED_FOLDERS);
}
