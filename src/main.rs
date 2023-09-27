use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = kluars::config::Cli::parse();

    kluars::run(cli).await
}
