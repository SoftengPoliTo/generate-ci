mod common;

use std::env::temp_dir;
use std::path::Path;

use generate_ci::{
    meson::{Meson, ProjectKind},
    CreateProject, TemplateData,
};

use common::compare_template;

const SNAPSHOT_PATH_CPP: &str = "../repositories/snapshots/meson_cpp/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/meson_c/";

#[test]
fn test_meson_cpp() {
    let tmp_dir = temp_dir();
    let path = tmp_dir.join("meson_cpp");
    let data = TemplateData::new(&path, "meson-cpp", "SoftengPoliTo", "test-meson-cpp")
        .license("BSD-1-Clause")
        .branch("main");

    Meson::new()
        .kind(ProjectKind::Cxx)
        .create_project(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_CPP), &path);
}

#[test]
fn test_meson_c() {
    let tmp_dir = temp_dir();
    let path = tmp_dir.join("meson_c");
    let data = TemplateData::new(&path, "meson-c", "SoftengPoliTo", "test-meson-c")
        .license("BSD-1-Clause")
        .branch("main");

    Meson::new()
        .kind(ProjectKind::C)
        .create_project(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_C), &path);
}
