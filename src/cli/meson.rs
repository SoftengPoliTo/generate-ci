use clap::{ArgMatches, Parser};

use figment::Figment;
use serde::{Deserialize, Serialize};

use ci_generate::meson::{Meson, ProjectKind};
use ci_generate::{CreateProject, TemplateData};

use crate::cli::{retrieve_data, CommonData};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub(crate) struct MesonData {
    /// Kind of a new meson project
    #[clap(long, short, value_parser = project_kind, default_value = "c")]
    kind: ProjectKind,
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

pub(super) fn meson_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let meson = retrieve_data::<MesonData>(config, matches, "meson")?;
    let data = TemplateData::new(&meson.common.project_path, &meson.common.name)
        .branch(&meson.common.branch)
        .license(&meson.common.license);
    Ok(Meson::new().kind(meson.kind).create_project(data)?)
}
