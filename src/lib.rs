use std::fs;

use anyhow::Result;
use config::{Cli, Global, LuaArgs};
use kube::{
    api::{Api, DynamicObject, Patch, PatchParams},
    core::{GroupVersionKind, TypeMeta},
    discovery::{ApiCapabilities, ApiResource, Discovery, Scope},
    Client,
};
use log::{info, trace, warn};
use mlua::{Lua, Table};

pub mod config;

fn run_lua<'lua>(lua: &'lua Lua, args: &LuaArgs) -> Result<Table<'lua>> {
    let LuaArgs {
        path, args, values, ..
    } = args;
    let script = if path.is_dir() {
        fs::read_to_string(path.join("init.lua"))?
    } else {
        fs::read_to_string(path)?
    };

    let globals = lua.globals();

    if let Some(values) = values {
        if values.is_file() {
            lua.load(values.clone())
                .set_name("Additional values")
                .exec()?;
        }
    }

    for (k, v) in args {
        globals.set(k.clone(), v.clone())?;
    }

    Ok(lua.load(&script).eval()?)
}

fn set_env(args: &LuaArgs) {
    let LuaArgs { path, .. } = args;

    if path.is_dir() {
        // Look for modules in the provided path
        std::env::set_var("LUA_PATH", path.join("?.lua"));
    }
}

fn translate_inner(args: Global) -> Result<String> {
    set_env(&args.lua_args);
    let mut out = String::new();
    let lua = Lua::new();
    let table = run_lua(&lua, &args.lua_args)?;

    if table.get::<_, Table>(1).is_ok() {
        // table is an array, produce multidoc yaml
        for pair in table.pairs::<u64, Table>() {
            let (_, v) = pair?;
            out += "---\n";
            out += &serde_yaml::to_string(&v)?;
        }
    } else {
        out += &serde_yaml::to_string(&table)?;
    }
    Ok(out)
}

fn translate(args: Global) -> Result<()> {
    println!("{}", translate_inner(args)?);
    Ok(())
}

async fn apply_single(
    args: &Global,
    table: &Table<'_>,
    client: &Client,
    discovery: &Discovery,
    ssapply: &PatchParams,
) -> Result<()> {
    let meta: Table = table.get("metadata")?;

    let namespace = meta
        .get::<&str, String>("namespace")
        .ok()
        .or(args.namespace.clone());
    let kind: String = table.get("kind")?;
    let api_version: String = table.get("apiVersion")?;
    let gvk = GroupVersionKind::try_from(TypeMeta { api_version, kind })?;
    let name: String = meta
        .get("name")
        .or_else(|_| meta.get("generateName"))
        .unwrap_or_default();

    if let Some((ar, caps)) = discovery.resolve_gvk(&gvk) {
        let api = dynamic_api(ar, caps, client.clone(), namespace.as_deref(), false);
        trace!(
            "Applying {}: \n{}",
            gvk.kind,
            serde_yaml::to_string(&table)?
        );
        let data: serde_json::Value = serde_json::to_value(table)?;
        let _r = api.patch(&name, ssapply, &Patch::Apply(data)).await?;
        info!("applied {} {}", gvk.kind, name);
    } else {
        warn!("Cannot apply document for unknown {:?}", gvk);
    }
    Ok(())
}

async fn apply(args: Global) -> Result<()> {
    let client = Client::try_default().await?;
    let discovery = Discovery::new(client.clone()).run().await?;
    let ssapply = PatchParams::apply("kubectl-light").force();

    set_env(&args.lua_args);
    let lua = Lua::new();
    let table = run_lua(&lua, &args.lua_args)?;

    if table.get::<_, Table>(1).is_ok() {
        // table is an array, iterate over entries
        for pair in table.pairs::<u64, Table>() {
            let (_, v) = pair?;
            apply_single(&args, &v, &client, &discovery, &ssapply).await?;
        }
    } else {
        apply_single(&args, &table, &client, &discovery, &ssapply).await?;
    }
    Ok(())
}

fn dynamic_api(
    ar: ApiResource,
    caps: ApiCapabilities,
    client: Client,
    ns: Option<&str>,
    all: bool,
) -> Api<DynamicObject> {
    if caps.scope == Scope::Cluster || all {
        Api::all_with(client, &ar)
    } else if let Some(namespace) = ns {
        Api::namespaced_with(client, namespace, &ar)
    } else {
        Api::default_namespaced_with(client, &ar)
    }
}

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        config::Commands::Xlate(args) => translate(args),
        config::Commands::Apply(args) => apply(args).await,
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use serde_yaml::Value;

    use super::*;

    #[test]
    fn basic() {
        let lua_args = LuaArgs {
            path: PathBuf::from("lua/pod.lua"),
            args: Vec::new(),
            values: None,
        };
        let args = Global {
            namespace: None,
            all: false,
            lua_args,
        };
        let name = &Value::String(String::from("nginx"));

        let out = translate_inner(args).expect("Failed to translate 'lua/pod.lua'");
        let out: HashMap<String, Value> = serde_yaml::from_str(&out).expect("Got invalid YAML");

        // kind: Pod
        let value = out.get("kind").expect("kind not found");
        let expected = &Value::String(String::from("Pod"));
        assert_eq!(value, expected);

        // apiVersion: v1
        let value = out.get("apiVersion").expect("apiVersion not found");
        let expected = &Value::String(String::from("v1"));
        assert_eq!(value, expected);

        // metadata
        let metadata = out.get("metadata").expect("did not find metadata");

        let value = metadata.get("name").expect("did not find name");
        let expected = name;
        assert_eq!(value, expected);

        // spec
        let spec = out.get("spec").expect("did not find spec");
        let containers = spec.get("containers").expect("did not find containers");
        let Value::Sequence(containers) = containers else {
            panic!("containers is not sequence");
        };
        assert_eq!(containers.len(), 1);

        let nginx = containers.get(0).unwrap();
        let value = nginx.get("name").expect("no name in nginx");
        assert_eq!(value, name);

        let value = nginx.get("image").expect("no image in nginx");
        let expected = &Value::String(String::from("nginx:1.14.2"));
        assert_eq!(value, expected);

        let ports = nginx.get("ports").expect("no ports in nginx");
        let Value::Sequence(ports) = ports else {
            panic!("ports is not sequence");
        };
        assert_eq!(ports.len(), 1);

        let port = ports.get(0).unwrap();
        let Value::Number(port) = port.get("containerPort").unwrap() else {
            panic!("containerPort is not number")
        };
        let port = port.as_u64().unwrap();
        assert_eq!(port, 80);
    }
}
