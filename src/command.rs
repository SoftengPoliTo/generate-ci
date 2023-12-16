use std::{
    path::Path,
    process::{Command, ExitStatus},
};

use crate::error::Result;

/// Runs cargo command
pub fn run_command(path: &Path, args: &[&str]) -> Result<ExitStatus> {
    let mut command = Command::new("cargo").args(args).arg(path).spawn()?;

    command.wait().map_err(|e| e.into())
}
