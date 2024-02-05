use clap::ArgMatches;

use figment::Figment;

use generate_ci::yarn::Yarn;
use generate_ci::{CreateCi, TemplateData};

use crate::CommonData;

use super::retrieve_data;

pub(crate) fn yarn_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let yarn = retrieve_data::<CommonData>(config, matches, "yarn")?;
    let data = TemplateData::new(&yarn.project_path, &yarn.name)
        .branch(&yarn.branch)
        .license(&yarn.license);
    Ok(Yarn::new().create_ci(data)?)
}
