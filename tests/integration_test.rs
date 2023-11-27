mod common;

#[test]
fn test_yarn_project() {
    // User creates a yarn project
    let _yarn_project = common::create_yarn_project();

    // All files created are encrypted in sha256
    let mut hash_file = vec![];
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/yarn_project/.github/workflows/yarn_project.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/yarn_project/.reuse/dep5")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/yarn_project/LICENSES/Apache-2.0.txt")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/yarn_project/.gitlab-ci.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/yarn_project/README.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );

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
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/.github/workflows/Myprog.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/.reuse/dep5")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/LICENSES/BSD-1-Clause.txt")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/Myprog/tests/__init__.py")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/Myprog/tests/test_sum.py")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/Myprog/__init__.py")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/Myprog/__main__.py")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/.pre-commit-config.yaml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/LICENSE.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/pyproject.toml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/poetry_project/README.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );

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
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/.github/workflows/meson_project.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/.reuse/dep5")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/cli/meson_project.c")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/cli/meson.build")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/lib/meson_project.c")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/lib/meson_project.h")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/lib/meson.build")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/LICENSES/APL-1.0.txt")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/tests/meson_project.c")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/tests/meson.build")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/docker-compose.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/Dockerfile")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/meson.build")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/README.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/meson_project/run_tests.sh")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );

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
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/.github/workflows/Myprog.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/.reuse/dep5")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/LICENSES/BSD-1-Clause.txt")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/src/main/java/POL/Myprog/Entry.java")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new(
                "tests/output/maven_project/src/test/java/POL/Myprog/example/Example.java",
            )
            .to_str()
            .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/checkstyle.xml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/LICENSE.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/pom.xml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/maven_project/README.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );

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
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/.github/workflows/deploy.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new(
                "tests/output/cargo_project/.github/workflows/Project-docker-application.yml",
            )
            .to_str()
            .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/.github/workflows/Project.yml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/.reuse/dep5")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/docker/Dockerfile-amd64")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/docker/Dockerfile-arm64")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/fuzz/fuzz_targets/fuzz_target_1.rs")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/fuzz/.gitignore")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/fuzz/Cargo.toml")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/LICENSES/EUPL-1.2.txt")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );
    hash_file.push(
        common::compute_sha256_hash_if_file(
            std::path::Path::new("tests/output/cargo_project/README.md")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
    );

    // Comparison with expected hash values
    for h in hash_file {
        assert!(common::hash_comparation(h, "cargo"));
    }
}
