mod common;

use std::path::PathBuf;

use common::*;

const REPO: &str = "tests/repositories/maven_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

const REPO_C: &str = "tests/repositories/config_template/maven_template_config/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/config/maven/";

const CONFIG_R: &str = "tests/repositories/config_template/config.toml";
const CONFIG_S: &str = "../repositories/snapshots/config/";

// $ ci-generate maven --license=BSD-1-Clause --branch=master POL tests/repositories/maven_template/
#[test]
fn test_maven() {
    compare_template_output_with_expected_one(&PathBuf::from(SNAPSHOT_PATH), &PathBuf::from(REPO));
}

// $ ci-generate -c tests/repositories/config_template/config.toml maven -l=APL-1.0 -b=master POL tests/repositories/config_template/maven_template_config
#[test]
fn test_maven_config() {
    compare_config_toml_wih_expected_one(&PathBuf::from(CONFIG_S), &PathBuf::from(CONFIG_R));
    compare_template_output_with_expected_one(
        &PathBuf::from(SNAPSHOT_PATH_C),
        &PathBuf::from(REPO_C),
    );
}
