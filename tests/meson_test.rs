mod common;

use common::compare_template_output_with_expected_one;

const REPO: &str = "tests/repositories/meson_template/";
const SNAPSHOT_PATH: &str = "../repositories/snapshots/meson/";

const REPO_C: &str = "tests/repositories/config_template/meson_template_config/";
const SNAPSHOT_PATH_C: &str = "../repositories/snapshots/config/meson/";

// $ ci-generate meson --kind=c --license=APL-1.0 --branch=main tests/repositories/meson_template
#[test]
fn test_meson() {
    compare_template_output_with_expected_one(SNAPSHOT_PATH, REPO);
}

// $ ci-generate -c tests/repositories/config_template/config.toml meson -l=APL-1.0 -b=master tests/repositories/config_template/meson_template_config
#[test]
fn test_meson_config() {
    compare_template_output_with_expected_one(SNAPSHOT_PATH_C, REPO_C);
}