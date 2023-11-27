mod common;

#[test]
fn test_yarn_project() {
    // User creates a yarn project
    let _yarn_project = common::create_yarn_project();

    // All files created are encrypted in sha256
    let mut hash_file = vec![];
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/yarn_project/.github/workflows/yarn_project.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/yarn_project/.reuse/dep5")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/yarn_project/LICENSES/Apache-2.0.txt")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/yarn_project/.gitlab-ci.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/yarn_project/README.md")).unwrap().to_str().unwrap()).unwrap());
   
    // Comparison with expected hash values
    for h in hash_file {
        assert!(common::hash_comparation(h, "yarn"));
    }
}

#[test]
fn test_poetry_project() {
    // User creates a poetry project
    let _yarn_project = common::create_poetry_project();

    // All files created are encrypted in sha256
    let mut hash_file = vec![];
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/.github/workflows/Myprog.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/.reuse/dep5")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/LICENSES/BSD-1-Clause.txt")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/Myprog/tests/__init__.py")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/Myprog/tests/test_sum.py")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/Myprog/__init__.py")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/Myprog/__main__.py")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/.pre-commit-config.yaml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/LICENSE.md")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/pyproject.toml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/poetry_project/README.md")).unwrap().to_str().unwrap()).unwrap());
    
    
    // Comparison with expected hash values
    for h in hash_file {
        assert!(common::hash_comparation(h, "poetry"));
    }
}

#[test]
fn test_meson_project() {
    // User creates a meson project
    let _meson_project = common::create_meson_project();

    // All files created are encrypted in sha256
    let mut hash_file = vec![];
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/.github/workflows/meson_project.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/.reuse/dep5")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/cli/meson_project.c")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/cli/meson.build")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/lib/meson_project.c")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/lib/meson_project.h")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/lib/meson.build")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/LICENSES/APL-1.0.txt")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/tests/meson_project.c")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/tests/meson.build")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/docker-compose.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/Dockerfile")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/meson.build")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/README.md")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/meson_project/run_tests.sh")).unwrap().to_str().unwrap()).unwrap());

    // Comparison with expected hash values
    for h in hash_file {
        assert!(common::hash_comparation(h, "meson"));
    }
}

#[test]
fn test_maven_project() {
    // User creates a maven project
    let _maven_project = common::create_maven_project();

   // All files created are encrypted in sha256
    let mut hash_file = vec![];
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/.github/workflows/Myprog.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/.reuse/dep5")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/LICENSES/BSD-1-Clause.txt")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/src/main/java/POL/Myprog/Entry.java")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/src/test/java/POL/Myprog/example/Example.java")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/checkstyle.xml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/LICENSE.md")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/pom.xml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/maven_project/README.md")).unwrap().to_str().unwrap()).unwrap());

    // Comparison with expected hash values
    for h in hash_file {
        assert!(common::hash_comparation(h, "maven"));
    }

 
}

#[test]
fn test_cargo_project() {
    // User creates a cargo project
    let _cargo_project = common::create_cargo_project();

    // All files created are encrypted in sha256
    let mut hash_file = vec![];
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/.github/workflows/deploy.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/.github/workflows/Project-docker-application.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/.github/workflows/Project.yml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/.reuse/dep5")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/docker/Dockerfile-amd64")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/docker/Dockerfile-arm64")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/fuzz/fuzz_targets/fuzz_target_1.rs")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/fuzz/.gitignore")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/fuzz/Cargo.toml")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/LICENSES/EUPL-1.2.txt")).unwrap().to_str().unwrap()).unwrap());
    hash_file.push(common::compute_sha256_hash_if_file(ci_generate::path_validation(std::path::Path::new("tests/output/cargo_project/README.md")).unwrap().to_str().unwrap()).unwrap());


    // Comparison with expected hash values
    for h in hash_file {
        assert!(common::hash_comparation(h, "cargo"));
    }
}