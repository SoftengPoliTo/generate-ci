use clap::ArgMatches;
use figment::Figment;

use generate_ci::poetry::Poetry;
use generate_ci::{CreateProject, TemplateData};

use crate::CommonData;

use super::retrieve_data;

pub(crate) fn poetry_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let poetry = retrieve_data::<CommonData>(config, matches, "poetry")?;
    let data = TemplateData::new(&poetry.project_path, &poetry.name, &poetry.organization)
        .branch(&poetry.branch)
        .license(&poetry.license);
    Ok(Poetry::new().create_project(data)?)
}
