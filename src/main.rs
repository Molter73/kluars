use std::{fs::read_to_string, path::Path};

use mlua::{Lua, Result, Table};

fn main() -> Result<()> {
    let lua = Lua::new();

    let globals = lua.globals();
    let script = read_to_string(Path::new("lua/pod.lua")).unwrap();
    lua.load(&script).set_name("pod.lua").exec()?;

    let obj = globals.get::<_, Table>("Obj")?;
    println!("{}", serde_yaml::to_string(&obj).unwrap());

    Ok(())
}
