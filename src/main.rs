use clap::Parser;
use mlua::Result;

fn main() -> Result<()> {
    let cli = kluars::config::Cli::parse();

    kluars::run(cli)
}
