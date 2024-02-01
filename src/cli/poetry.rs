use clap::ArgMatches;
use figment::Figment;

use ci_generate::poetry::Poetry;
use ci_generate::{CreateProject, TemplateData};

use crate::cli::{retrieve_data, CommonData};

pub(super) fn poetry_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let poetry = retrieve_data::<CommonData>(config, matches, "poetry")?;
    let data = TemplateData::new(&poetry.project_path, &poetry.name)
        .branch(&poetry.branch)
        .license(&poetry.license);
    Ok(Poetry::new().create_project(data)?)
}
