mod common;
use common::*;

use ci_generate::error::Result;
use std::{fs, path::Path};

const REPO: &str = "tests/repositories/tmp_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

#[test]
fn test_meson() {
    fs::create_dir(REPO).unwrap();
    create_meson_project().unwrap();
    compare_template_output_with_expected_one(Path::new(SNAPSHOT_PATH), Path::new(REPO));
}

fn create_meson_project() -> Result<()> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::meson::Meson::new(ci_generate::toolchain::meson::ProjectKind::C),
        "",
        Path::new(REPO),
        "BSD-3-Clause",
        "main",
    )
}
