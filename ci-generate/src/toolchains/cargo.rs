use clap::{ArgMatches, Parser};

use figment::Figment;
use serde::{Deserialize, Serialize};

use generate_ci::cargo::Cargo;
use generate_ci::{CreateCi, TemplateData};

use crate::CommonData;

use super::retrieve_data;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub(crate) struct CargoData {
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

pub(crate) fn cargo_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let cargo = retrieve_data::<CargoData>(config, matches, "cargo")?;
    let docker_image_description = cargo.docker_image_description.map_or_else(
        || format!("{} description", &cargo.common.name),
        |desc| desc,
    );
    let data = TemplateData::new(&cargo.common.project_path, &cargo.common.name)
        .branch(&cargo.common.branch)
        .license(&cargo.common.license);
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
