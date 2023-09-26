use std::fs::read_to_string;

use config::Cli;
use mlua::{Lua, Result, Table};

pub mod config;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        config::Commands::Xlate { script, args } => {
            let lua = Lua::new();

            let globals = lua.globals();

            for (k, v) in args {
                globals.set(k, v)?;
            }

            let script = read_to_string(script)?;
            let res: Table = lua.load(&script).set_name("pod.lua").eval()?;

            println!("{}", serde_yaml::to_string(&res).unwrap());
        }
    }
    Ok(())
}
