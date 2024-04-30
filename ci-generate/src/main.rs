mod toolchains;

use std::path::PathBuf;

use anyhow::anyhow;
use clap::parser::ValueSource;
use clap::{ArgMatches, FromArgMatches, Parser};
use figment::providers::Serialized;
use figment::value::{Dict, Map, Value};
use figment::Profile;
use figment::{Metadata, Provider};
use serde::{Deserialize, Serialize};

use clap::CommandFactory;
use figment::providers::{Format, Toml};
use figment::Figment;

use tracing_subscriber::EnvFilter;

use toolchains::*;

#[derive(clap::Parser, Debug)]
enum Cmd {
    #[cfg(feature = "cargo")]
    /// Generate a CI for a cargo project.
    Cargo(cargo::CargoData),
    #[cfg(feature = "maven")]
    /// Generate a new maven project
    Maven(maven::MavenData),
    #[cfg(feature = "meson")]
    /// Generate a new meson project
    Meson(meson::MesonData),
    #[cfg(feature = "poetry")]
    /// Generate a new poetry project.
    Poetry(CommonData),
    #[cfg(feature = "yarn")]
    /// Generate a new yarn project.
    Yarn(CommonData),
}

fn choose_commands(config: Figment, sub: (&str, &ArgMatches)) -> anyhow::Result<()> {
    match sub {
        #[cfg(feature = "cargo")]
        ("cargo", matches) => cargo::cargo_config(config, matches),
        #[cfg(feature = "maven")]
        ("maven", matches) => maven::maven_config(config, matches),
        #[cfg(feature = "meson")]
        ("meson", matches) => meson::meson_config(config, matches),
        #[cfg(feature = "poetry")]
        ("poetry", matches) => poetry::poetry_config(config, matches),
        #[cfg(feature = "yarn")]
        ("yarn", matches) => yarn::yarn_config(config, matches),
        _ => unreachable!("unexpected command"),
    }
}

static DEFAULT_CONF: &str = r#"
    [default]
    license = "MIT"
    branch = "main"

    [meson]
    kind = "c"

    [cargo]
    lib = false
    ci = false
"#;

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
    /// Project name
    #[clap(long)]
    name: String,
    /// Path to the new project
    #[clap(value_hint = clap::ValueHint::DirPath)]
    project_path: PathBuf,
    /// Organization name 
    #[clap(long, short = 'o')]
    organization: String,
}

struct ClapSerialized<T> {
    serialized: Serialized<T>,
    matches: ArgMatches,
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

fn local_config() -> anyhow::Result<PathBuf> {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            home::home_dir()
                .map(|home| home.join(".config"))
                .ok_or_else(|| anyhow!("Cannot find the home directory"))
        })?;

    Ok(config_dir.join("ci-generate").join("config.toml"))
}

fn create_config() -> anyhow::Result<()> {
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

    let config = Figment::new()
        .merge(Toml::string(DEFAULT_CONF).nested())
        .merge(Toml::file(config_file).nested());

    let sub = matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Missing command"))?;

    choose_commands(config, sub)
}

fn main() -> anyhow::Result<()> {
    create_config()
}
