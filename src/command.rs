use std::{
    path::Path,
    process::{Child, Command},
};

use crate::error::Result;

/// Runs cargo command
pub fn run_command(path: &Path, args: &[&str]) -> Result<()> {
    let command = Command::new("cargo")
        .args(args)
        .arg(path)
        .spawn()
        .expect("couldn't run &args");

    handle_child_process(command)?;
    Ok(())
}

fn handle_child_process(mut child: Child) -> Result<()> {
    child.wait()?;
    Ok(())
}
