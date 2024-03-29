use std::collections::HashMap;
use std::path::{Path, PathBuf};

use minijinja::value::Value;

use crate::{
    builtin_templates, error::Result, BuildTemplate, CreateProject, ProjectOutput, TemplateData,
};

use super::create_toolchain;

static POETRY_TEMPLATES: &[(&str, &str)] = &builtin_templates!["poetry" =>
    ("toml.pyproject", "pyproject.toml"),
    ("yaml.pre-commit", ".pre-commit-config.yaml"),
    ("md.README", "README.md"),
    ("py.__init__", "__init__.py"),
    ("py.__main__", "__main__.py"),
    ("py.test", "test_sum.py"),
    ("ci.github", "github.yml"),
    ("ci.github.dependabot", "dependabot.yml")
];

/// A poetry project data.
#[derive(Default)]
pub struct Poetry;

impl CreateProject for Poetry {
    fn create_project(&self, data: TemplateData) -> Result<()> {
        create_toolchain(self, data)
    }
}

impl Poetry {
    /// Creates a new `Poetry` instance.
    pub fn new() -> Self {
        Self
    }

    fn project_structure(
        project_path: &Path,
        name: &str,
    ) -> (HashMap<PathBuf, &'static str>, Vec<PathBuf>) {
        let root = project_path.to_path_buf();
        let main = project_path.join(name);
        let data = project_path.join(format!("{name}/data"));
        let tests = project_path.join(format!("{name}/tests"));
        let github = project_path.join(".github");
        let workflows = github.join("workflows");

        let mut template_files = HashMap::new();

        // All the files in the root of the projects
        template_files.insert(root.join("pyproject.toml"), "toml.pyproject");
        template_files.insert(root.join(".pre-commit-config.yaml"), "yaml.pre-commit");
        template_files.insert(root.join("README.md"), "md.README");
        template_files.insert(root.join("LICENSE.md"), "build.license");

        // All files in the main directory
        template_files.insert(main.join("__init__.py"), "py.__init__");
        template_files.insert(main.join("__main__.py"), "py.__main__");

        // All files in the tests/ directory
        template_files.insert(tests.join("__init__.py"), "py.__init__");
        template_files.insert(tests.join("test_sum.py"), "py.test");

        // dependabot
        template_files.insert(github.join("dependabot.yml"), "ci.github.dependabot");

        // Continuous integration files
        template_files.insert(workflows.join(format!("{name}.yml")), "ci.github");

        (template_files, vec![root, main, data, tests, workflows])
    }
}

impl BuildTemplate for Poetry {
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

        let (files, dirs) = Poetry::project_structure(project_path, project_name);

        Ok(ProjectOutput {
            context,
            files,
            dirs,
        })
    }

    fn get_templates() -> &'static [(&'static str, &'static str)] {
        POETRY_TEMPLATES
    }
}
