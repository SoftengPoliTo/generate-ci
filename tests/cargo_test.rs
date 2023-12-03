mod common;

use std::path::PathBuf;

use common::*;

const REPO: &str = "tests/repositories/cargo_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/cargo/";

const REPO_C: &str = "tests/repositories/config_template/cargo_template_config/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/config/cargo/";

const CONFIG_R: &str = "tests/repositories/config_template/config.toml";
const CONFIG_S: &str = "../repositories/snapshots/config/";

// $ ci-generate cargo --docker-image-description=docker_image --license=EUPL-1.2 --name=Project --branch=main tests/repositories/cargo_template
#[test]
fn test_cargo() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}

// $ ci-generate -c tests/repositories/config_template/config.toml cargo --docker-image-description=docker_image tests/repositories/config_template/cargo_template_config
#[test]
fn test_cargo_config() {
    compare_config_toml_wih_expected_one(&PathBuf::from(CONFIG_S), &PathBuf::from(CONFIG_R));
    compare_template_output_with_expected_one(
        &PathBuf::from(SNAPSHOT_PATH_C),
        &PathBuf::from(REPO_C),
    );
}
