mod common;

use std::env::temp_dir;
use std::path::Path;

use ci_generate::{maven::Maven, CreateProject, TemplateData};

use common::compare_template;

const SNAPSHOT_PATH: &str = "../repositories/snapshots/maven/";

#[test]
fn test_maven() {
    let tmp_dir = temp_dir();
    let path = tmp_dir.join("maven");
    let data = TemplateData::new(&path, "maven-java")
        .license("BSD-1-Clause")
        .branch("main");

    Maven::new().group("POL").create_project(data).unwrap();
    compare_template(Path::new(SNAPSHOT_PATH), &path);
}
