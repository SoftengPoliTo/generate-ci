pub mod toolchain;
pub use toolchain::*;

pub mod error;
use error::{Error, Result};

mod command;

mod filters;

use minijinja::value::Value;
use minijinja::Environment;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};
use tracing::debug;

use filters::*;

static REUSE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", "dep5"));

#[derive(Debug)]
pub struct TemplateData<'a> {
    license: Cow<'a, str>,
    branch: Cow<'a, str>,
    name: Cow<'a, str>,
    project_path: &'a Path,
}
impl<'a> TemplateData<'a> {
    /// Creates a new `Common` instance.
    pub fn new(project_path: &'a Path) -> Self {
        Self {
            license: "MIT".into(),
            branch: "main".into(),
            name: "".into(),
            project_path,
        }
    }
    /// Sets a new license.
    pub fn license(mut self, license: impl Into<Cow<'a, str>>) -> Self {
        self.license = license.into();
        self
    }

    /// Sets a new branch.
    pub fn branch(mut self, branch: impl Into<Cow<'a, str>>) -> Self {
        self.branch = branch.into();
        self
    }

    /// Sets a new project_name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.name = name.into();
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

        license_ctx.insert("header", Value::from_serializable(&header));
        license_ctx.insert("text", Value::from_serializable(&text_without_blank));
        license_ctx.insert("id", Value::from_serializable(&id));

        self.context
            .insert("license", Value::from_serializable(&license_ctx));

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

        reuse.insert("name", Value::from_serializable(&name));
        reuse.insert("id", Value::from_serializable(&id));

        self.context
            .insert("reuse", Value::from_serializable(&reuse));

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
    ) -> Result<ProjectOutput>;

    fn get_templates() -> &'static [(&'static str, &'static str)];

    fn build(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
        github_branch: &str,
    ) -> Result<CiTemplate> {
        let t = self.define(project_path, project_name, license, github_branch)?;
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
// Retrieve the project name
pub(crate) fn define_name<'a>(project_name: &'a str, project_path: &'a Path) -> Result<&'a str> {
    Ok(if !project_name.is_empty() && project_name.is_ascii() {
        project_name
    } else {
        project_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(Error::UTF8Check)?
    })
}

// Retrieve the license
pub(crate) fn define_license(license: &str) -> Result<&dyn license::License> {
    license.parse::<&dyn license::License>().map_err(|_| {
        if license.is_empty() {
            Error::NoLicense
        } else {
            Error::InvalidLicense
        }
    })
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
    if project_path.ends_with(".") {
        return std::env::current_dir().map_err(|e| e.into());
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

    // Canonicalize project path parent and create a more correct path
    let project_path = match project_path.parent() {
        Some(parent) => {
            if parent.ends_with("") {
                project_path.canonicalize()?
            } else {
                let canonical_parent = parent.canonicalize()?;
                canonical_parent.join(project_path.file_name().ok_or(Error::NoDirectory)?)
            }
        }
        None => project_path,
    };

    // Create missing directories
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
                Err(Error::NoLicense) => prop_assert!(data.license_str.is_empty()),
                Err(Error::InvalidLicense) => prop_assert!(!VALID_LICENSES.contains(&data.license_str.as_str())),
                Ok(_) => prop_assert!(VALID_LICENSES.contains(&data.license_str.as_str())),
                //This branch is made general to consider all other error cases in the error.rs library,
                //but which in the context of this API will never be called.
                _ => {},
            }
        }
    }

    fn path_strategy() -> impl Strategy<Value = PathBuf> {
        "\\PC*".prop_map(PathBuf::from)
    }

    proptest! {
        #[test]
        fn define_name_proptest(project_name in "\\PC*", project_path in path_strategy()) {
            let project_path_str_option = project_path.file_name().and_then(|x| x.to_str());

            match define_name(&project_name, &project_path) {
                Ok(name) => {
                    if !project_name.is_empty() && project_name.is_ascii() {
                        prop_assert_eq!(name, &project_name)
                    } else {
                        prop_assert_eq!(Some(name), project_path_str_option)
                    }
                }
                Err(Error::UTF8Check) => {
                    prop_assert!(project_path_str_option.map_or(true, |s| s.is_empty() || !s.is_ascii() || s.contains('/')))
                }
                // This branch is made general to consider all other error cases in the error.rs library,
                // but which in the context of this API will never be called.
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

    #[cfg(windows)]
    #[test]
    fn test_invalid_home_directory() {
        let project_path = Path::new("~\\subfolder");
        assert!(matches!(path_validation(project_path), Err(Error::HomeDir)));
    }

    #[test]
    fn test_invalid_utf8_path() {
        let invalid_utf8 = String::from_utf8_lossy(&[0xC3, 0x28]).into_owned();
        let project_path = Path::new(&invalid_utf8);
        path_validation(project_path).unwrap();
        //assert!(matches!(path_validation(project_path), Err(Error::Io(_))));
    }
}
