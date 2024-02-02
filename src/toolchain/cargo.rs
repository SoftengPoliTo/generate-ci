use minijinja::value::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::{
    builtin_templates, command::run_command, error::Result, BuildTemplate, CreateCi, ProjectOutput,
    TemplateData,
};

use super::create_toolchain;

static CARGO_TEMPLATES: &[(&str, &str)] = &builtin_templates!["cargo" =>
    ("md.README", "README.md"),
    ("ci.github", "github.yml"),
    ("ci.github.deploy", "github-deploy.yml"),
    ("ci.github.docker", "github-docker-application.yml"),
    ("ci.github.dependabot", "dependabot.yml"),
    ("docker.amd64", "Dockerfile-amd64"),
    ("docker.arm64", "Dockerfile-arm64"),
    ("rs.proptest", "proptest.rs"),
    // Execute checks and tests locally through xtask
    ("xtask.config", "xtask-config"),
    ("xtask.toml", "xtask/Cargo.toml"),
    // Tasks
    ("xtask.src.books", "xtask/src/books.rs"),
    ("xtask.src.dependencies", "xtask/src/dependencies.rs"),
    ("xtask.src.logging", "xtask/src/logging.rs"),
    ("xtask.src.main", "xtask/src/main.rs"),
    ("xtask.src.publish", "xtask/src/publish.rs"),
    ("xtask.src.runchecks", "xtask/src/runchecks.rs"),
    ("xtask.src.vulnerabilities", "xtask/src/vulnerabilities.rs"),
    // Utilities
    ("xtask.src.utils.cargo", "xtask/src/utils/cargo.rs"),
    ("xtask.src.utils.mdbook", "xtask/src/utils/mdbook.rs"),
    ("xtask.src.utils.mod", "xtask/src/utils/mod.rs"),
    ("xtask.src.utils.process", "xtask/src/utils/process.rs"),
    ("xtask.src.utils.rustup", "xtask/src/utils/rustup.rs"),
    ("xtask.src.utils.time", "xtask/src/utils/time.rs"),
    ("xtask.src.utils.workspace", "xtask/src/utils/workspace.rs")
];

/// A cargo project data.
#[derive(Default)]
pub struct Cargo<'a> {
    docker_image_description: &'a str,
    ci: bool,
    lib: bool,
}

impl<'a> CreateCi for Cargo<'a> {
    fn create_ci(&self, data: TemplateData) -> Result<()> {
        create_toolchain(self, data)
    }
}

impl<'a> Cargo<'a> {
    /// Creates a new `Cargo` instance.
    pub fn new() -> Self {
        Self {
            docker_image_description: "default",
            lib: false,
            ci: false,
        }
    }

    /// Sets a description
    pub fn docker_image_description(mut self, docker_image_description: &'a str) -> Self {
        self.docker_image_description = docker_image_description;
        self
    }

    /// Sets a library project
    pub fn create_lib(mut self) -> Self {
        self.lib = true;
        self
    }

    /// Sets just ci files
    pub fn only_ci(mut self) -> Self {
        self.ci = true;
        self
    }

    fn project_creation(&self, path: &Path) -> Result<()> {
        if !self.ci {
            if self.lib {
                run_command(path, &["init", "--lib"])?;
            } else {
                run_command(path, &["init"])?;
            }
            run_command(
                &path.join("Cargo.toml"),
                &["add", "--dev", "proptest", "--manifest-path"],
            )?;
        }
        Ok(())
    }

    fn project_structure(
        project_path: &Path,
        name: &str,
        ci: bool,
    ) -> (HashMap<PathBuf, &'static str>, Vec<PathBuf>) {
        let root = project_path.to_path_buf();
        let github = project_path.join(".github");
        let workflows = github.join("workflows");
        let docker = project_path.join("docker");
        let cargo = project_path.join(".cargo");
        let xtask = project_path.join("xtask");
        let xtask_src = xtask.join("src");
        let xtask_utils = xtask_src.join("utils");

        let mut template_files = HashMap::new();

        // README
        template_files.insert(root.join("README.md"), "md.README");

        // dependabot
        template_files.insert(github.join("dependabot.yml"), "ci.github.dependabot");

        // Continuous Integration
        template_files.insert(workflows.join(format!("{name}.yml")), "ci.github");
        template_files.insert(
            workflows.join(format!("{name}-docker-application.yml")),
            "ci.github.docker",
        );
        template_files.insert(workflows.join("deploy.yml"), "ci.github.deploy");

        // Docker
        template_files.insert(docker.join("Dockerfile-amd64"), "docker.amd64");
        template_files.insert(docker.join("Dockerfile-arm64"), "docker.arm64");

        // xtask configuration file for cargo
        template_files.insert(cargo.join("config"), "xtask.config");

        // xtask
        template_files.insert(xtask.join("Cargo.toml"), "xtask.toml");
        // xtask/src
        template_files.insert(xtask_src.join("books.rs"), "xtask.src.books");
        template_files.insert(xtask_src.join("dependencies.rs"), "xtask.src.dependencies");
        template_files.insert(xtask_src.join("logging.rs"), "xtask.src.logging");
        template_files.insert(xtask_src.join("main.rs"), "xtask.src.main");
        template_files.insert(xtask_src.join("publish.rs"), "xtask.src.publish");
        template_files.insert(xtask_src.join("runchecks.rs"), "xtask.src.runchecks");
        template_files.insert(
            xtask_src.join("vulnerabilities.rs"),
            "xtask.src.vulnerabilities",
        );
        // xtask/src/utils
        template_files.insert(xtask_utils.join("cargo.rs"), "xtask.src.utils.cargo");
        template_files.insert(xtask_utils.join("mdbook.rs"), "xtask.src.utils.mdbook");
        template_files.insert(xtask_utils.join("mod.rs"), "xtask.src.utils.mod");
        template_files.insert(xtask_utils.join("process.rs"), "xtask.src.utils.process");
        template_files.insert(xtask_utils.join("rustup.rs"), "xtask.src.utils.rustup");
        template_files.insert(xtask_utils.join("time.rs"), "xtask.src.utils.time");
        template_files.insert(
            xtask_utils.join("workspace.rs"),
            "xtask.src.utils.workspace",
        );

        if !ci {
            // Proptest
            let tests = project_path.join("tests");
            template_files.insert(tests.join("proptest.rs"), "rs.proptest");
            (
                template_files,
                vec![root, workflows, docker, cargo, xtask_utils, tests],
            )
        } else {
            (
                template_files,
                vec![root, workflows, docker, cargo, xtask_utils],
            )
        }
    }
}

impl<'a> BuildTemplate for Cargo<'a> {
    fn define(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
        github_branch: &str,
    ) -> Result<ProjectOutput> {
        let mut context = HashMap::new();

        context.insert("name", Value::from_serializable(&project_name));
        context.insert("branch", Value::from_serializable(&github_branch));
        context.insert("license_id", Value::from_serializable(&license));
        context.insert(
            "docker_image_description",
            Value::from_serializable(&self.docker_image_description),
        );

        Cargo::project_creation(self, project_path)?;

        let (files, dirs) = Cargo::project_structure(project_path, project_name, self.ci);

        Ok(ProjectOutput {
            files,
            dirs,
            context,
        })
    }

    fn get_templates() -> &'static [(&'static str, &'static str)] {
        CARGO_TEMPLATES
    }
}
