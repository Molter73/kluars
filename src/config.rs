use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Xlate {
        script: PathBuf,

        #[arg(short, long, value_parser=parse_key_val::<String, String>)]
        args: Vec<(String, String)>,

        #[arg(short, long)]
        values: Option<PathBuf>,
    },
}

// Shamelessly stolen from:
// https://github.com/clap-rs/clap/blob/204552890d316ec9ae0b21f85298ba1d5d0786f8/examples/typed-derive.rs#L47-L59
/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
