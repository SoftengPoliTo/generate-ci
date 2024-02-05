#[cfg(feature = "cargo")]
pub(crate) mod cargo;
#[cfg(feature = "maven")]
pub(crate) mod maven;
#[cfg(feature = "meson")]
pub(crate) mod meson;
#[cfg(feature = "poetry")]
pub(crate) mod poetry;
#[cfg(feature = "yarn")]
pub(crate) mod yarn;

fn retrieve_data<'a, T: serde::Deserialize<'a>>(
    config: figment::Figment,
    matches: &clap::ArgMatches,
    toolchain: &str,
) -> anyhow::Result<T> {
    let config = config
        .merge(crate::ClapSerialized::<crate::CommonData>::globals(
            matches.clone(),
        ))
        .select(toolchain);
    config.extract::<T>().map_err(|e| e.into())
}
