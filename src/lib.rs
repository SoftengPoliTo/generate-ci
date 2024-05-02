pub mod toolchain;
pub use toolchain::*;

pub mod error;
use error::{Error, Result};

mod command;

mod filters;

use minijinja::value::Value;
use minijinja::Environment;
use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};
use tracing::debug;

use filters::*;

static REUSE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", "dep5"));

#[derive(Debug)]
pub struct TemplateData<'a> {
    project_path: &'a Path,
    name: &'a str,
    license: &'a str,
    branch: &'a str,
    organization: &'a str,
    repository: &'a str,
}
impl<'a> TemplateData<'a> {
    /// Creates a new `Common` instance.
    pub fn new(project_path: &'a Path, name: &'a str, organization: &'a str, repository: &'a str) -> Self {
        Self {
            project_path,
            name,
            license: "MIT",
            branch: "main",
            organization,
            repository,
        }
    }
    /// Sets a new license.
    pub fn license(mut self, license: &'a str) -> Self {
        self.license = license;
        self
    }

    /// Sets a new branch.
    pub fn branch(mut self, branch: &'a str) -> Self {
        self.branch = branch;
        self
    }
}

/// Used to create a CI configuration for a project.
pub trait CreateCi {
    /// Creates a new CI configuration for a project.
    fn create_ci(&self, data: TemplateData) -> Result<()>;
}

/// Used to create a new project.
pub trait CreateProject {
    /// Creates a new project.
    fn create_project(&self, data: TemplateData) -> Result<()>;
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
            create_dir_all(dir)?;
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

    fn add_license(&mut self, license: &dyn license::License, project_path: &Path) -> Result<()> {
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

        license_ctx.insert("header", Value::from_serialize(header));
        license_ctx.insert("text", Value::from_serialize(text_without_blank));
        license_ctx.insert("id", Value::from_serialize(id));

        self.context
            .insert("license", Value::from_serialize(license_ctx));

        self.env.add_template("build.license", license.text())?;

        Ok(())
    }

    fn add_reuse(&mut self, license: &dyn license::License, project_path: &Path) -> Result<()> {
        // Adds .reuse directory and dep5 file
        let reuse_path = project_path.join(".reuse");
        self.files.insert(reuse_path.join("dep5"), "dep5.reuse");
        self.dirs.push(reuse_path);

        // Gets project name and license header
        let name = self.context.get("name");
        let id = license.id();

        let mut reuse = HashMap::new();

        reuse.insert("name", Value::from_serialize(name));
        reuse.insert("id", Value::from_serialize(id));

        self.context.insert("reuse", Value::from_serialize(reuse));

        self.env.add_template("dep5.reuse", REUSE_TEMPLATE)?;

        Ok(())
    }
}

struct ProjectOutput {
    files: HashMap<PathBuf, &'static str>,
    dirs: Vec<PathBuf>,
    context: HashMap<&'static str, Value>,
}

/// Build a template
trait BuildTemplate {
    fn define(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
        github_branch: &str,
        organization: &str,
        repository: &str,
    ) -> Result<ProjectOutput>;

    fn get_templates() -> &'static [(&'static str, &'static str)];

    fn build(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
        github_branch: &str,
        organization: &str,
        repository: &str,
    ) -> Result<CiTemplate> {
        let t = self.define(
            project_path,
            project_name,
            license,
            github_branch,
            organization,
            repository,
        )?;
        let env = build_environment(Self::get_templates());

        Ok(CiTemplate {
            context: t.context,
            files: t.files,
            dirs: t.dirs,
            env,
        })
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

// Retrieve the license
pub(crate) fn define_license(license: &str) -> Result<&dyn license::License> {
    license
        .parse::<&dyn license::License>()
        .map_err(|e| e.into())
}

// Compute template
pub(crate) fn compute_template(
    mut template: CiTemplate,
    license: &dyn license::License,
    project_path: &Path,
) -> Result<()> {
    template.add_reuse(license, project_path)?;
    template.add_license(license, project_path)?;
    template.render()
}

// Performs path validation
pub fn path_validation(project_path: &Path) -> Result<PathBuf> {
    // Do not accept a file, only a directory
    if project_path.is_file() {
        return Err(Error::NoDirectory);
    }

    // If only the "." value is passed, returns the current path
    if project_path.as_os_str() == "." {
        return std::env::current_dir().map_err(|e| e.into());
    }

    // Check whether the path contains valid UTF-8 characters
    if project_path.to_str().map_or(true, |s| s.contains('�')) {
        return Err(Error::Utf8Check);
    }

    // Get a different home prefix according to different operating systems
    let prefix = if cfg!(windows) { r#"~\"# } else { "~" };

    // Get home directory
    let project_path = if project_path.starts_with(prefix) {
        home::home_dir()
            .ok_or(Error::HomeDir)?
            .join(project_path.strip_prefix(prefix)?)
    } else {
        project_path.to_path_buf()
    };

    // Create directories recursively when they do not exist
    create_dir_all(&project_path)?;

    Ok(project_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;
    use std::env;

    static VALID_LICENSES: [&str; 3] = ["MIT", "Apache-2.0", "GPL-3.0"];

    #[derive(Debug, Arbitrary)]
    struct LicenseTest {
        license_str: String,
    }

    proptest! {
        #[test]
        fn define_license_proptest(data: LicenseTest) {

            match define_license(&data.license_str) {
                Ok(_) => prop_assert!(VALID_LICENSES.contains(&data.license_str.as_str())),
                Err(Error::InvalidLicense(_)) => prop_assert!(!VALID_LICENSES.contains(&data.license_str.as_str())),
                // All other use-cases are not considered
                _ => {},
            }
        }
    }

    #[test]
    fn test_invalid_path_file() {
        let repo_path = env::var("CARGO_MANIFEST_DIR")
            .expect("Unable to retrieve the environment variable CARGO_MANIFEST_DIR!");
        let project_path = format!("{}/src/lib.rs", repo_path);

        assert!(matches!(
            path_validation(Path::new(&project_path)),
            Err(Error::NoDirectory)
        ));
    }

    #[test]
    fn test_valid_path_folder() {
        let repo_path = env::var("CARGO_MANIFEST_DIR")
            .expect("Unable to retrieve the environment variable CARGO_MANIFEST_DIR!");
        let project_path = format!("{}/src", repo_path);

        assert!(path_validation(Path::new(&project_path)).is_ok());
    }

    #[test]
    fn test_current_path() {
        assert!(path_validation(Path::new(".")).is_ok());
    }

    // Test for path validation for windows
    #[cfg(windows)]
    #[test]
    fn test_valid_path_windows() {
        let valid_path = Path::new("C:\\user\\docs\\Letter.txt");
        assert!(path_validation(&valid_path).is_ok());
    }

    #[test]
    fn test_invalid_utf8_path() {
        let invalid_utf8 = String::from_utf8_lossy(&[0xC3, 0x28]).into_owned();
        let project_path = Path::new(&invalid_utf8);
        assert!(matches!(
            path_validation(project_path),
            Err(Error::Utf8Check)
        ));
    }
}
