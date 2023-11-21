pub mod toolchain;
pub use toolchain::*;

mod filters;

use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use minijinja::value::Value;
use minijinja::Environment;
use tracing::debug;

use filters::*;

static REUSE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", "dep5"));

/// Used to create a CI configuration for a project.
pub trait CreateCi {
    /// Creates a new CI configuration for a project.
    fn create_ci(
        &self,
        project_name: &str,
        project_path: &Path,
        license: &str,
        github_branch: &str,
    ) -> Result<()>;
}

/// Used to create a new project.
pub trait CreateProject {
    /// Creates a new project.
    fn create_project(
        &self,
        project_name: &str,
        project_path: &Path,
        license: &str,
        github_branch: &str,
    ) -> Result<()>;
}

struct CiTemplate {
    context: HashMap<&'static str, Value>,
    files: HashMap<PathBuf, &'static str>,
    dirs: Vec<PathBuf>,
    env: Environment<'static>,
}

impl CiTemplate {
    fn render(self) -> Result<()> {
        //let mut env = Environment::new();
        let CiTemplate {
            context,
            files,
            dirs,
            mut env,
        } = self;

        // Create dirs
        for dir in dirs {
            debug!("Creating {}", dir.display());
            create_dir_all(dir)?
        }

        env.add_filter("comment_license", comment_license);
        env.add_filter("hypens_to_underscores", hypens_to_underscores);

        // Fill in templates
        for (path, template_name) in files {
            debug!("Creating {}", path.display());
            let template = env.get_template(template_name)?;
            let filled_template = template.render(&context)?;
            write(path, filled_template)?;
        }

        Ok(())
    }

    fn add_license(
        &mut self,
        license: &dyn license::License,
        project_path: &Path,
    ) -> anyhow::Result<()> {
        let id = license.id();
        let header = license.header();

        // Adds LICENSE directory and license file
        let license_path = project_path.join("LICENSES");
        self.files
            .insert(license_path.join(format!("{}.txt", id)), "build.license");
        self.dirs.push(license_path);

        let text_without_blank: Vec<&str> = license
            .text()
            .lines()
            .skip(2) // Skip a blank line and license id
            .filter(|x| !x.is_empty())
            .collect();

        let mut license_ctx = HashMap::new();

        license_ctx.insert("header", Value::from_serializable(&header));
        license_ctx.insert("text", Value::from_serializable(&text_without_blank));
        license_ctx.insert("id", Value::from_serializable(&id));

        self.context
            .insert("license", Value::from_serializable(&license_ctx));

        self.env.add_template("build.license", license.text())?;

        Ok(())
    }

    fn add_reuse(
        &mut self,
        license: &dyn license::License,
        project_path: &Path,
    ) -> anyhow::Result<()> {
        // Adds .reuse directory and dep5 file
        let reuse_path = project_path.join(".reuse");
        self.files.insert(reuse_path.join("dep5"), "dep5.reuse");
        self.dirs.push(reuse_path);

        // Gets project name and license header
        let name = self.context.get("name");
        let id = license.id();

        let mut reuse = HashMap::new();

        reuse.insert("name", Value::from_serializable(&name));
        reuse.insert("id", Value::from_serializable(&id));

        self.context
            .insert("reuse", Value::from_serializable(&reuse));

        self.env.add_template("dep5.reuse", REUSE_TEMPLATE)?;

        Ok(())
    }
}

/// Build a template
trait BuildTemplate {
    fn define(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
        github_branch: &str,
    ) -> (
        HashMap<PathBuf, &'static str>,
        Vec<PathBuf>,
        HashMap<&'static str, Value>,
    );

    fn get_templates() -> &'static [(&'static str, &'static str)];

    fn build(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
        github_branch: &str,
    ) -> CiTemplate {
        let (files, dirs, context) =
            self.define(project_path, project_name, license, github_branch);
        let env = build_environment(Self::get_templates());

        CiTemplate {
            context,
            files,
            dirs,
            env,
        }
    }
}

fn build_environment(templates: &'static [(&'static str, &'static str)]) -> Environment<'static> {
    let mut environment = Environment::new();
    for (name, src) in templates {
        environment
            .add_template(name, src)
            .expect("Internal error, built-in template");
    }

    environment
}

pub(crate) fn define_name<'a>(project_name: &'a str, project_path: &'a Path) -> Result<&'a str> {
    let project_name = if project_name.is_empty() {
        if let Some(os_name) = project_path.file_name() {
            if let Some(name) = os_name.to_str() {
                name
            } else {
                bail!("Impossible to convert the project name into a valid Unicode string");
            }
        } else {
            bail!("Impossible to get the project name");
        }
    } else {
        project_name
    };

    Ok(project_name)
}

pub(crate) fn define_license(license: &str) -> Result<&dyn license::License> {
    let license = license
        .parse::<&dyn license::License>()
        .map_err(|_| anyhow::anyhow!("Cannot find License"))?;
    Ok(license)
}

pub(crate) fn compute_template(
    mut template: CiTemplate,
    license: &dyn license::License,
    project_path: &Path,
) -> Result<()> {
    template.add_reuse(license, project_path)?;
    template.add_license(license, project_path)?;

    template.render()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cargo::Cargo, maven::Maven, meson::Meson, poetry::Poetry, yarn::Yarn};

    // Test on CiTemplate functions
    fn creator_citemplate() -> CiTemplate {
        CiTemplate {
            context: HashMap::new(),
            files: HashMap::new(),
            dirs: Vec::new(),
            env: Environment::new(),
        }
    }

    #[test]
    fn citemplate_add_license_test() {
        assert!(creator_citemplate()
            .add_license(
                "Apache-2.0".parse().unwrap(),
                Path::new("/home/user/project")
            )
            .is_ok());
    }
    #[test]
    fn citemplate_add_reuse_test() {
        assert!(creator_citemplate()
            .add_reuse(
                "Apache-2.0".parse().unwrap(),
                Path::new("/home/user/project")
            )
            .is_ok());
    }
    #[test]
    fn citemplate_render_test() {
        assert!(creator_citemplate().render().is_ok());
    }

    // Test other lib internal functions
    #[test]
    fn define_name_valid_test() {
        assert!(define_name("test-project", Path::new("~/Desktop/project")).is_ok());
    }
    #[test]
    fn define_name_emptyname_test() {
        assert!(define_name("", Path::new("~/Desktop/project")).is_ok());
    }
    #[test]
    fn define_name_emptypath_test() {
        assert!(define_name("", Path::new("")).is_err());
    }
    #[test]
    fn define_license_valid_test() {
        assert!(define_license("AFL-3.0").is_ok())
    }
    #[test]
    fn define_license_invalid_test() {
        assert!(define_license("POL-3.0").is_err())
    }
    #[test]
    #[ignore]
    fn compute_template_test() {
        assert!(compute_template(
            creator_citemplate(),
            "Apache-2.0".parse().unwrap(),
            Path::new("~/Desktop/foo.txt")
        )
        .is_ok())
    }

    #[test]
    fn build_environment_test() {
        assert!(build_environment(&[("index.html", "Hello {{ name }} !")])
            .add_template("index.html", "Hello {{ name }} !")
            .is_ok());
    }

    // Tests for trait BildTemplate - Yarn
    fn create_yarn() -> Yarn {
        Yarn::new()
    }
    #[test]
    fn build_contain_files_test_yarn() {
        assert!(create_yarn()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .files
            .contains_key(Path::new("~/Desktop/project/README.md")));
    }
    #[test]
    fn build_dirs_test_yarn() {
        assert_eq!(
            create_yarn()
                .build(
                    Path::new("~/Desktop/project"),
                    "my_prog",
                    "Apache-2.0",
                    "master"
                )
                .dirs,
            vec![
                Path::new("~/Desktop/project"),
                Path::new("~/Desktop/project/.github/workflows")
            ]
        )
    }
    #[test]
    fn build_fullcontext_test_yarn() {
        assert!(!create_yarn()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .context
            .is_empty())
    }

    // Tests for BilfTemplate trait - Poetry
    fn create_poetry() -> Poetry {
        Poetry::new()
    }

    #[test]
    fn build_contain_files_test_poetry() {
        assert!(create_poetry()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .files
            .contains_key(Path::new("~/Desktop/project/README.md")))
    }
    #[test]
    fn build_dirs_test_poetry() {
        assert_eq!(
            create_poetry()
                .build(
                    Path::new("~/Desktop/project"),
                    "my_prog",
                    "Apache-2.0",
                    "master"
                )
                .dirs,
            vec![
                Path::new("~/Desktop/project"),
                Path::new("~/Desktop/project/my_prog"),
                Path::new("~/Desktop/project/my_prog/data"),
                Path::new("~/Desktop/project/my_prog/tests"),
                Path::new("~/Desktop/project/.github/workflows")
            ]
        )
    }
    #[test]
    fn build_fullcontext_test_poetry() {
        assert!(!create_poetry()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .context
            .is_empty())
    }

    // Tests for BilfTemplate trait - Meson
    fn create_meson() -> Meson {
        Meson::new(meson::ProjectKind::C)
    }

    #[test]
    fn build_contain_files_test_meson() {
        assert!(create_meson()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .files
            .contains_key(Path::new("~/Desktop/project/README.md")))
    }
    #[test]
    fn build_dirs_test_meson() {
        assert_eq!(
            create_meson()
                .build(
                    Path::new("~/Desktop/project"),
                    "my_prog",
                    "Apache-2.0",
                    "master"
                )
                .dirs,
            vec![
                Path::new("~/Desktop/project"),
                Path::new("~/Desktop/project/cli"),
                Path::new("~/Desktop/project/lib"),
                Path::new("~/Desktop/project/tests"),
                Path::new("~/Desktop/project/.github/workflows")
            ]
        )
    }
    #[test]
    fn build_fullcontext_test_meson() {
        assert!(!create_meson()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .context
            .is_empty())
    }

    // Tests for BilfTemplate trait - Maven
    fn create_maven() -> Maven<'static> {
        Maven::new("group_name")
    }

    #[test]
    fn build_contain_files_test_maven() {
        assert!(create_maven()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .files
            .contains_key(Path::new("~/Desktop/project/README.md")))
    }
    #[test]
    fn build_content_dirs_test_maven() {
        assert_eq!(
            create_maven()
                .build(
                    Path::new("~/Desktop/project"),
                    "my_prog",
                    "Apache-2.0",
                    "master"
                )
                .dirs,
            vec![
                Path::new("~/Desktop/project"),
                Path::new("~/Desktop/project/src/main/java/group_name/my_prog"),
                Path::new("~/Desktop/project/src/test/java/group_name/my_prog/example"),
                Path::new("~/Desktop/project/.github/workflows")
            ]
        )
    }
    #[test]
    fn build_fullcontext_test_maven() {
        assert!(!create_maven()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .context
            .is_empty())
    }

    // Tests for BilfTemplate trait - Cargo
    fn create_cargo() -> Cargo<'static> {
        Cargo::new("docker_image_description")
    }

    #[test]
    fn build_contain_files_test_cargo() {
        assert!(create_cargo()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .files
            .contains_key(Path::new("~/Desktop/project/README.md")))
    }
    #[test]
    fn build_content_dirs_test_cargo() {
        assert_eq!(
            create_cargo()
                .build(
                    Path::new("~/Desktop/project"),
                    "my_prog",
                    "Apache-2.0",
                    "master"
                )
                .dirs,
            vec![
                Path::new("~/Desktop/project"),
                Path::new("~/Desktop/project/.github/workflows"),
                Path::new("~/Desktop/project/docker"),
                Path::new("~/Desktop/project/fuzz"),
                Path::new("~/Desktop/project/fuzz/fuzz_targets")
            ]
        )
    }
    #[test]
    fn build_fullcontext_test_cargo() {
        assert!(!create_maven()
            .build(
                Path::new("~/Desktop/project"),
                "my_prog",
                "Apache-2.0",
                "master"
            )
            .context
            .is_empty())
    }
}
