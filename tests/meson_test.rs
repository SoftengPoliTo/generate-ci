mod common;

use std::path::PathBuf;

use common::*;

const REPO: &str = "tests/repositories/meson_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

const REPO_C: &str = "tests/repositories/config_template/meson_template_config/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/config/meson/";

const CONFIG_R: &str = "tests/repositories/config_template/config.toml";
const CONFIG_S: &str = "../repositories/snapshots/config/";

// $ ci-generate meson --kind=c --license=APL-1.0 --branch=main tests/repositories/meson_template
#[test]
fn test_meson() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}

// $ ci-generate -c tests/repositories/config_template/config.toml meson -l=APL-1.0 -b=master tests/repositories/config_template/meson_template_config
#[test]
fn test_meson_config() {
    compare_config_toml_wih_expected_one(&PathBuf::from(CONFIG_S), &PathBuf::from(CONFIG_R));
    compare_template_output_with_expected_one(
        &PathBuf::from(SNAPSHOT_PATH_C),
        &PathBuf::from(REPO_C),
    );
}
