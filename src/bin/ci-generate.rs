#[path = "../cli/mod.rs"]
mod cli;

use cli::create_config;

fn main() -> anyhow::Result<()> {
    create_config()
}
