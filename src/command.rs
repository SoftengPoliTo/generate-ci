use tracing::debug;

use crate::error::Result;
use std::{str,
    path::Path,
    process::{Command, ExitStatus},
};

/// Runs cargo command
pub(crate) fn run_command(path: &Path, args: &[&str]) -> Result<ExitStatus> {
    let output = Command::new("cargo").args(args).arg(path).output()?;

    debug!("{:#?}", str::from_utf8(&output.stdout).map_err(|e| e));

    Ok(output.status)
}
