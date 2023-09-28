use std::{error::Error, path::PathBuf};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
pub struct LuaArgs {
    /// Path to lua script or a directory holding init.lua
    pub path: PathBuf,

    /// Global variables to be added into lua env, can be used multiple times
    #[arg(short, long, value_parser=parse_key_val::<String, String>)]
    pub args: Vec<(String, String)>,

    /// A lua script that will be run before the main one for defining globals
    #[arg(short = 'g', long = "globals")]
    pub values: Option<PathBuf>,
}

#[derive(Args)]
pub struct Global {
    /// k8s namespace to be used
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Unused
    #[arg(short = 'A', long = "all-namespaces")]
    pub all: bool,

    /// Arguments to pass into lua
    #[command(flatten)]
    pub lua_args: LuaArgs,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Translate lua scripts to YAML
    Xlate(Global),
    /// Apply lua configuration to k8s cluster
    Apply(Global),
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
