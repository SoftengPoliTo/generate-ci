mod common;
use common::compare_template;

use ci_generate::{
    meson::{Meson, ProjectKind},
    CreateProject, TemplateData,
};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH_CPP: &str = "../repositories/snapshots/meson_cpp/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/meson_c/";

#[test]
fn test_meson_cpp() {
    let tmp_dir = temp_dir().join("meson_cpp");
    let data = TemplateData::new(&tmp_dir)
        .license("BSD-1-Clause")
        .branch("main")
        .name("Meson-project");

    Meson::new()
        .kind(ProjectKind::Cxx)
        .create_project(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_CPP), &tmp_dir);
}

#[test]
fn test_meson_c() {
    let tmp_dir = temp_dir().join("meson_c");
    let data = TemplateData::new(&tmp_dir)
        .license("BSD-1-Clause")
        .branch("main")
        .name("Meson-project");

    Meson::new()
        .kind(ProjectKind::C)
        .create_project(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_C), &tmp_dir);
}
