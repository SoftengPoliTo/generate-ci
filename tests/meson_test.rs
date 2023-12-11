mod common;
use common::compare_template;

use ci_generate::{
    meson::{Meson, MesonData, ProjectKind},
    CommonData, CreateProject,
};
use std::env::temp_dir;
use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

#[test]
#[ignore]
fn test_meson() {
    let tmp_dir = temp_dir().join("meson");
    let meson_data = MesonData::new(ProjectKind::Cxx);
    let common_data = CommonData::new("BSD-1-Clause", "main", "Meson-project", &tmp_dir);
    Meson::new(meson_data.kind)
        .create_project(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &tmp_dir);
}
