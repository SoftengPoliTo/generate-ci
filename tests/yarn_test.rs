mod common;

use std::path::PathBuf;

use common::*;

const REPO: &str = "tests/repositories/yarn_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

const REPO_C: &str = "tests/repositories/config_template/meson_template_config/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/config/meson/";

const CONFIG_R: &str = "tests/repositories/config_template/config.toml";
const CONFIG_S: &str = "../repositories/snapshots/config/";

// $ ci-generate yarn --license=Apache-2.0 --name=Test_Project --branch=main tests/repositories/yarn_template
#[test]
fn test_yarn() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}

// $ ci-generate -c tests/repositories/config_template/config.toml yarn --license=Apache-2.0 --name=Test_Project --branch=main tests/repositories/config_template/yarn_template_config
#[test]
fn test_yarn_config() {
    compare_config_toml_wih_expected_one(&PathBuf::from(CONFIG_S), &PathBuf::from(CONFIG_R));
    compare_template_output_with_expected_one(
        &PathBuf::from(SNAPSHOT_PATH_C),
        &PathBuf::from(REPO_C),
    );
}
