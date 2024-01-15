use std::path::PathBuf;

use anyhow::anyhow;
use clap::parser::ValueSource;
use clap::{ArgMatches, CommandFactory, FromArgMatches, Parser};
use figment::providers::{Format, Serialized, Toml};
use figment::value::{Dict, Map, Value};
use figment::{Figment, Profile};
use figment::{Metadata, Provider};
use serde::{Deserialize, Serialize};

use ci_generate::{CreateCi, CreateProject, TemplateData};

use ci_generate::cargo::Cargo;
use ci_generate::maven::Maven;
use ci_generate::meson::{Meson, ProjectKind};
use ci_generate::poetry::Poetry;
use ci_generate::yarn::Yarn;

use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
struct Opts {
    /// Use the configuration file instead the one located in ${XDG_CONFIG_HOME}/ci-generate
    #[clap(short, long, value_hint = clap::ValueHint::FilePath)]
    config: Option<PathBuf>,
    /// Output the generated paths as they are produced
    #[clap(short, long, global = true)]
    verbose: bool,
    #[clap(subcommand)]
    cmd: Cmd,
}

fn from_id(id: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    id.parse::<&dyn license::License>()
        .map(|_| id.to_owned())
        .map_err(|_| "License not found".into())
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct CommonData {
    /// License to be used in the project
    #[clap(long, short, value_parser = from_id, default_value = "MIT")]
    license: String,
    /// GitHub branch name to be used in the project
    #[clap(long, short = 'b', default_value = "main")]
    branch: String,
    /// Override the project name
    #[clap(long, default_value = "")]
    name: String,
    /// Path to the new project
    #[clap(value_hint = clap::ValueHint::DirPath)]
    project_path: PathBuf,
}

static DEFAULT_CONF: &str = r#"
    [default]
    license = "MIT"
    branch = "main"
    name = ""

    [meson]
    kind = "c"

    [cargo]
    lib = false
    ci = false
"#;

struct ClapSerialized<T> {
    pub serialized: Serialized<T>,
    pub matches: ArgMatches,
}

impl<T> ClapSerialized<T>
where
    T: FromArgMatches + Serialize,
{
    fn globals(matches: ArgMatches) -> Self {
        let t = <T as FromArgMatches>::from_arg_matches(&matches).expect("Clap mismatch error");

        let serialized = Serialized::globals(t);

        Self {
            serialized,
            matches,
        }
    }
}

impl<T: Serialize> Provider for ClapSerialized<T> {
    fn metadata(&self) -> Metadata {
        self.serialized.metadata()
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        let value = Value::serialize(&self.serialized.value)?;
        let tag = value.tag();
        let error = figment::error::Kind::InvalidType(value.to_actual(), "map".into());

        let mut dict = value.into_dict().ok_or(error.clone())?;

        self.matches
            .ids()
            .filter_map(|id| {
                let id = id.as_str();
                match self.matches.value_source(id) {
                    Some(ValueSource::DefaultValue) => Some(id),
                    _ => None,
                }
            })
            .for_each(|id| {
                dict.remove(id);
            });

        let value = Value::Dict(tag, dict);
        let dict = match &self.serialized.key {
            Some(key) => figment::util::nest(key, value).into_dict().ok_or(error)?,
            None => value.into_dict().ok_or(error)?,
        };

        Ok(self.serialized.profile.clone().collect(dict))
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct CargoData {
    /// Docker image description.
    #[clap(long)]
    docker_image_description: Option<String>,
    /// Used for creating a library project
    #[clap(long, global = false)]
    lib: bool,
    /// Used for creating just cargo ci files
    #[clap(long, global = false)]
    ci: bool,
    #[clap(flatten)]
    #[serde(flatten)]
    common: CommonData,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct MesonData {
    /// Kind of a new meson project
    #[clap(long, short, value_parser = project_kind, default_value = "c")]
    kind: ProjectKind,
    #[clap(flatten)]
    #[serde(flatten)]
    common: CommonData,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct MavenData {
    /// Java group.
    group: String,
    #[clap(flatten)]
    #[serde(flatten)]
    common: CommonData,
}

fn project_kind(
    s: &str,
) -> Result<ProjectKind, Box<dyn std::error::Error + Send + Sync + 'static>> {
    match s {
        "c" => Ok(ProjectKind::C),
        "c++" => Ok(ProjectKind::Cxx),
        _ => Err(format!("{s} is not a valid meson project kind.").into()),
    }
}

#[derive(Parser, Debug)]
enum Cmd {
    /// Generate a CI for a cargo project.
    Cargo(CargoData),
    /// Generate a new maven project
    Maven(MavenData),
    /// Generate a new meson project
    Meson(MesonData),
    /// Generate a new poetry project.
    Poetry(CommonData),
    /// Generate a new yarn project.
    Yarn(CommonData),
}

fn local_config() -> anyhow::Result<PathBuf> {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            dirs::home_dir()
                .map(|home| home.join(".config"))
                .ok_or_else(|| anyhow!("Cannot find the home directory"))
        })?;

    Ok(config_dir.join("ci-generate").join("config.toml"))
}

fn main() -> anyhow::Result<()> {
    let cmd = Opts::command();
    let matches = cmd.get_matches();
    let verbose = matches.get_flag("verbose");

    let config_file = if let Some(cfg) = matches.get_one::<PathBuf>("config") {
        cfg.to_owned()
    } else {
        local_config()?
    };

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            if verbose {
                EnvFilter::try_new("debug")
            } else {
                EnvFilter::try_new("info")
            }
        })
        .unwrap();

    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(filter_layer)
        .with_writer(std::io::stderr)
        .init();

    let sub = matches
        .subcommand()
        .ok_or_else(|| anyhow!("Missing command"))?;

    let config = Figment::new()
        .merge(Toml::string(DEFAULT_CONF).nested())
        .merge(Toml::file(config_file).nested());

    match sub {
        ("cargo", matches) => {
            let config = config
                .merge(ClapSerialized::<CargoData>::globals(matches.clone()))
                .select("cargo");
            let cargo: CargoData = config.extract()?;
            let docker_image_description = cargo.docker_image_description.map_or_else(
                || format!("{} description", &cargo.common.name),
                |desc| desc,
            );
            let data = TemplateData::new(&cargo.common.project_path)
                .branch(&cargo.common.branch)
                .license(&cargo.common.license)
                .name(&cargo.common.name);
            if cargo.ci {
                Ok(Cargo::new()
                    .docker_image_description(&docker_image_description)
                    .only_ci()
                    .create_ci(data)?)
            } else if !cargo.ci && cargo.lib {
                Ok(Cargo::new()
                    .docker_image_description(&docker_image_description)
                    .create_lib()
                    .create_ci(data)?)
            } else {
                Ok(Cargo::new()
                    .docker_image_description(&docker_image_description)
                    .create_ci(data)?)
            }
        }
        ("maven", matches) => {
            let config = config
                .merge(ClapSerialized::<MavenData>::globals(matches.clone()))
                .select("maven");
            let maven: MavenData = config.extract()?;
            let data = TemplateData::new(&maven.common.project_path)
                .branch(&maven.common.branch)
                .license(&maven.common.license)
                .name(&maven.common.name);
            Ok(Maven::new().group(&maven.group).create_project(data)?)
        }
        ("meson", matches) => {
            let config = config
                .merge(ClapSerialized::<MesonData>::globals(matches.clone()))
                .select("meson");
            let meson: MesonData = config.extract()?;
            let data = TemplateData::new(&meson.common.project_path)
                .branch(&meson.common.branch)
                .license(&meson.common.license)
                .name(&meson.common.name);
            Ok(Meson::new().kind(meson.kind).create_project(data)?)
        }
        ("poetry", matches) => {
            let config = config
                .merge(ClapSerialized::<CommonData>::globals(matches.clone()))
                .select("poetry");
            let poetry: CommonData = config.extract()?;
            let data = TemplateData::new(&poetry.project_path)
                .branch(&poetry.branch)
                .license(&poetry.license)
                .name(&poetry.name);
            Ok(Poetry::new().create_project(data)?)
        }
        ("yarn", matches) => {
            let config = config
                .merge(ClapSerialized::<CommonData>::globals(matches.clone()))
                .select("yarn");
            let yarn: CommonData = config.extract()?;
            let data = TemplateData::new(&yarn.project_path)
                .branch(&yarn.branch)
                .license(&yarn.license)
                .name(&yarn.name);
            Ok(Yarn::new().create_ci(data)?)
        }
        _ => unreachable!("unexpected command"),
    }
}
