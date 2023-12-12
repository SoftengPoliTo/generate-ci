mod common;
use common::compare_template;

use ci_generate::{
    meson::{Meson, ProjectKind},
    CreateProject, TemplateData,
};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

#[test]
fn test_meson() {
    let tmp_dir = temp_dir().join("meson");
    let data = TemplateData::new(&tmp_dir)
        .license("BSD-1-Clause")
        .branch("main")
        .name("Meson-project");

    Meson::new()
        .kind(ProjectKind::Cxx)
        .create_project(
            data
            /*data.get_name(),
            data.get_path(),
            data.get_license(),
            data.get_branch(),*/
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
