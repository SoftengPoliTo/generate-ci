mod common;
use common::compare_template;

use ci_generate::{
    meson::{Meson, ProjectKind},
    CreateProject, TemplateData,
};
use std::path::Path;
use std::{env::temp_dir, fs::create_dir_all};

const SNAPSHOT_PATH_CPP: &str = "../repositories/snapshots/meson_cpp/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/meson_c/";

#[test]
fn test_meson_cpp() {
    let tmp_dir = temp_dir();
    create_dir_all(tmp_dir.join("meson_cpp")).unwrap();
    let path = tmp_dir.join("meson_cpp");
    let data = TemplateData::new(&path)
        .license("BSD-1-Clause")
        .branch("main")
        .name("Meson-project");

    Meson::new()
        .kind(ProjectKind::Cxx)
        .create_project(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_CPP), &path);
}

#[test]
fn test_meson_c() {
    let tmp_dir = temp_dir();
    create_dir_all(tmp_dir.join("meson_c")).unwrap();
    let path = tmp_dir.join("meson_c");
    let data = TemplateData::new(&path)
        .license("BSD-1-Clause")
        .branch("main")
        .name("Meson-project");

    Meson::new()
        .kind(ProjectKind::C)
        .create_project(data)
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH_C), &path);
}
