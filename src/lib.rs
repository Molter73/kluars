use std::fs;

use anyhow::Result;
use config::{Cli, Global};
use kube::{
    api::{Api, DynamicObject, Patch, PatchParams},
    core::{GroupVersionKind, TypeMeta},
    discovery::{ApiCapabilities, ApiResource, Discovery, Scope},
    Client,
};
use log::{info, trace, warn};
use mlua::{Lua, Table};

pub mod config;

fn run_lua<'lua>(lua: &'lua Lua, args: &Global) -> Result<Table<'lua>> {
    let Global {
        path, args, values, ..
    } = args;
    let script = if path.is_dir() {
        // Look for modules in the provided path
        std::env::set_var("LUA_PATH", path.join("?.lua"));
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

fn translate(args: Global) -> Result<()> {
    let lua = Lua::new();
    let res = run_lua(&lua, &args)?;

    println!("{}", serde_yaml::to_string(&res).unwrap());
    Ok(())
}

async fn apply(args: Global) -> Result<()> {
    let client = Client::try_default().await?;
    let discovery = Discovery::new(client.clone()).run().await?;
    let ssapply = PatchParams::apply("kubectl-light").force();
    let lua = Lua::new();
    let table = run_lua(&lua, &args)?;

    let namespace = table
        .get::<&str, Table>("metadata")?
        .get::<&str, String>("namespace")
        .ok()
        .or(args.namespace);
    let kind: String = table.get("kind")?;
    let api_version: String = table.get("apiVersion")?;
    let gvk = GroupVersionKind::try_from(TypeMeta { api_version, kind })?;
    let name: String = table.get::<_, Table>("metadata")?.get("name")?;

    if let Some((ar, caps)) = discovery.resolve_gvk(&gvk) {
        let api = dynamic_api(ar, caps, client.clone(), namespace.as_deref(), false);
        trace!(
            "Applying {}: \n{}",
            gvk.kind,
            serde_yaml::to_string(&table)?
        );
        let data: serde_json::Value = serde_json::to_value(&table)?;
        let _r = api.patch(&name, &ssapply, &Patch::Apply(data)).await?;
        info!("applied {} {}", gvk.kind, name);
    } else {
        warn!("Cannot apply document for unknown {:?}", gvk);
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
