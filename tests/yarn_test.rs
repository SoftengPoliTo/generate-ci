mod common;

use common::compare_template_output_with_expected_one;

const REPO: &str = "tests/repositories/yarn_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/yarn/";

const REPO_C: &str = "tests/repositories/config_template/yarn_template_config/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/config/yarn/";

// $ ci-generate yarn --license=Apache-2.0 --name=Test_Project --branch=main tests/repositories/yarn_template
#[test]
fn test_yarn() {
    compare_template_output_with_expected_one(SNAPSHOT_PATH, REPO);
}

// $ ci-generate --config tests/repositories/config_template/config.toml yarn tests/repositories/config_template/yarn_template_config
#[test]
fn test_yarn_config() {
    compare_template_output_with_expected_one(SNAPSHOT_PATH_C, REPO_C);
}
