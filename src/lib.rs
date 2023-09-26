use std::fs;

use config::Cli;
use mlua::{Lua, Result, Table};

pub mod config;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        config::Commands::Xlate { script, args } => {
            let script = if script.is_dir() {
                // Look for modules in the provided path
                std::env::set_var("LUA_PATH", script.join("?.lua"));
                fs::read_to_string(script.join("init.lua"))?
            } else {
                fs::read_to_string(script)?
            };

            let lua = Lua::new();

            let globals = lua.globals();

            for (k, v) in args {
                globals.set(k, v)?;
            }

            let res: Table = lua.load(&script).eval()?;

            println!("{}", serde_yaml::to_string(&res).unwrap());
        }
    }
    Ok(())
}
