mod common;
use common::compare_template;

use ci_generate::{
    meson::{Meson, MesonData, ProjectKind},
    CommonData, CreateProject,
};
use std::path::Path;

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

#[test]
fn test_meson() {
    let meson_data = MesonData::new(ProjectKind::Cxx);
    let common_data = CommonData::new("BSD-1-Clause", "main", "Meson-project", Path::new(REPO));
    Meson::new(meson_data.kind)
        .create_project(
            common_data.name,
            common_data.project_path,
            common_data.license,
            common_data.branch,
        )
        .unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}
