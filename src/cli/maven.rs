use clap::{ArgMatches, Parser};

use figment::Figment;
use serde::{Deserialize, Serialize};

use ci_generate::maven::Maven;
use ci_generate::{CreateProject, TemplateData};

use crate::cli::{retrieve_data, CommonData};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub(crate) struct MavenData {
    /// Java group.
    group: String,
    #[clap(flatten)]
    #[serde(flatten)]
    common: CommonData,
}

pub(super) fn maven_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let maven = retrieve_data::<MavenData>(config, matches, "maven")?;
    let data = TemplateData::new(&maven.common.project_path, &maven.common.name)
        .branch(&maven.common.branch)
        .license(&maven.common.license);
    Ok(Maven::new().group(&maven.group).create_project(data)?)
}
