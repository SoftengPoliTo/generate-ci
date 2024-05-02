pub mod cargo;
pub mod maven;
pub mod meson;
pub mod poetry;
pub mod yarn;

use crate::{
    compute_template, define_license, error::Result, path_validation, BuildTemplate, TemplateData,
};

macro_rules! builtin_templates {
    ($root:expr => $(($name:expr, $template:expr)),+) => {
        [
        $(
            (
                $name,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/", $root, "/", $template)),
            )
        ),+
        ]
    }
}

pub(crate) use builtin_templates;

fn create_toolchain<T: BuildTemplate>(toolchain: &T, data: TemplateData) -> Result<()> {
    let project_path = path_validation(data.project_path)?;
    let license = define_license(data.license)?;
    let template = toolchain.build(
        &project_path,
        data.name,
        license.id(),
        data.branch,
        data.organization,
        data.repository,
    );
    compute_template(template?, license, &project_path)
}
