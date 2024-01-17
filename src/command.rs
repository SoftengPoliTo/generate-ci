use tracing::debug;

use crate::error::Result;
use std::{
    path::Path,
    process::{Command, ExitStatus},
    str,
};

/// Runs cargo command
pub(crate) fn run_command(path: &Path, args: &[&str]) -> Result<ExitStatus> {
    let output = Command::new("cargo").args(args).arg(path).output()?;

    debug!(
        "{:#?}",
        str::from_utf8(&output.stdout).unwrap_or("Error converting bytes into a characters.")
    );

    Ok(output.status)
}
