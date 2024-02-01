use clap::ArgMatches;

use figment::Figment;

use ci_generate::yarn::Yarn;
use ci_generate::{CreateCi, TemplateData};

use crate::cli::{retrieve_data, CommonData};

pub(super) fn yarn_config(config: Figment, matches: &ArgMatches) -> anyhow::Result<()> {
    let yarn = retrieve_data::<CommonData>(config, matches, "yarn")?;
    let data = TemplateData::new(&yarn.project_path, &yarn.name)
        .branch(&yarn.branch)
        .license(&yarn.license);
    Ok(Yarn::new().create_ci(data)?)
}
