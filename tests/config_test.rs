mod common;
use common::*;

use std::path::Path;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/config/";

#[test]
#[ignore]
fn test_config_toml() {
    compare_config_toml_wih_expected_one(Path::new(SNAPSHOT_PATH));
}
