mod common;
use common::compare_template;

use ci_generate::{
    meson::{Meson, ProjectKind},
    CommonData, CreateProject,
};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

#[test]
fn test_meson() {
    let tmp_dir = temp_dir().join("meson");
    let meson_data = Meson::new().kind(ProjectKind::Cxx);
    let common_data = CommonData::new()
        .license("BSD-1-Clause")
        .branch("main")
        .name("Meson-project")
        .project_path(&tmp_dir);

    meson_data
        .create_project(
            common_data.get_name(),
            common_data.get_path(),
            common_data.get_license(),
            common_data.get_branch(),
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
