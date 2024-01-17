use crate::error::Result;
use std::{
    path::Path,
    process::{Command, ExitStatus},
};
use tracing::debug;

/// Runs cargo command
pub(crate) fn run_command(path: &Path, args: &[&str]) -> Result<ExitStatus> {
    let output = Command::new("cargo").args(args).arg(path).output()?;

    let stdout_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr_str = String::from_utf8_lossy(&output.stderr).trim().to_string();

    #[cfg(debug_assertions)]
    {
        debug!("Command output (stdout): {}", stdout_str);
        if !output.status.success() {
            debug!("Command output (stderr): {}", stderr_str);
        }
    }

    Ok(output.status)
}
