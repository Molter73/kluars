use std::{collections::HashMap, process::Command};

use anyhow::Result;
use assert_cmd::prelude::*;
use serde_yaml::{with::singleton_map_recursive::deserialize, Value};

#[test]
fn basic() -> Result<()> {
    let output = Command::cargo_bin("kluars")?
        .args(["xlate", "lua/pod.lua"])
        .output()?;
    assert!(output.status.success());
    let out = String::from_utf8(output.stdout)?;
    let out: HashMap<String, Value> = serde_yaml::from_str(&out).expect("Got invalid YAML");

    let name = &Value::String(String::from("nginx"));

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

    let nginx = &containers[0];
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

    let port = &ports[0];
    let Value::Number(port) = port.get("containerPort").unwrap() else {
        panic!("containerPort is not number")
    };
    let port = port.as_u64().unwrap();
    assert_eq!(port, 80);
    Ok(())
}

#[test]
fn global_template() -> Result<()> {
    let output = Command::cargo_bin("kluars")?
        .args([
            "xlate",
            "-g",
            "lua/template/values.lua",
            "lua/template/pod.lua",
        ])
        .output()?;
    assert!(output.status.success());
    let out = String::from_utf8(output.stdout)?;
    let out: HashMap<String, Value> = serde_yaml::from_str(&out).expect("Got invalid YAML");
    let name = &Value::String(String::from("web"));

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

    let nginx = &containers[0];
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

    let port = &ports[0];
    let Value::Number(port) = port.get("containerPort").unwrap() else {
        panic!("containerPort is not number")
    };
    let port = port.as_u64().unwrap();
    assert_eq!(port, 8080);

    Ok(())
}

#[test]
fn args_over_global() -> Result<()> {
    let output = Command::cargo_bin("kluars")?
        .args([
            "xlate",
            "-g",
            "lua/template/values.lua",
            "-a",
            "name=something",
            "-a",
            "port=42069",
            "lua/template/pod.lua",
        ])
        .output()?;
    assert!(output.status.success());
    let out = String::from_utf8(output.stdout)?;
    let out: HashMap<String, Value> = serde_yaml::from_str(&out).expect("Got invalid YAML");
    let name = &Value::String(String::from("something"));

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

    let nginx = &containers[0];
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

    let port = &ports[0];
    let Value::Number(port) = port.get("containerPort").unwrap() else {
        panic!("containerPort is not number")
    };
    let port = port.as_u64().unwrap();
    assert_eq!(port, 42069);
    Ok(())
}

#[test]
fn two_containers_pod() -> Result<()> {
    let output = Command::cargo_bin("kluars")?
        .args(["xlate", "lua/two-container-pod/"])
        .output()?;
    assert!(output.status.success());
    let out = String::from_utf8(output.stdout)?;
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
    let expected = &Value::String(String::from("two-containers"));
    assert_eq!(value, expected);

    // spec
    let spec = out.get("spec").expect("did not find spec");
    let value = spec.get("restartPolicy").expect("no restartPolicy in spec");
    let expected = &Value::String(String::from("Never"));
    assert_eq!(value, expected);

    let volumes = spec.get("volumes").expect("no volumes in spec");
    let Value::Sequence(volumes) = volumes else {
        panic!("volumes is not sequence");
    };
    assert_eq!(volumes.len(), 1);
    let volume = &volumes[0];

    let value = volume.get("name").expect("no name in volume");
    let expected = &Value::String(String::from("shared-data"));
    assert_eq!(value, expected);

    let empty_dir = volume.get("emptyDir").expect("no emptyDir in volume");
    let Value::Mapping(empty_dir) = empty_dir else {
        panic!("emptyDir is not sequence");
    };
    assert!(empty_dir.is_empty());

    let containers = spec.get("containers").expect("did not find containers");
    let Value::Sequence(containers) = containers else {
        panic!("containers is not sequence");
    };
    assert_eq!(containers.len(), 2);

    // nginx container
    let nginx = &containers[0];
    let value = nginx.get("name").expect("no name in nginx");
    let expected = &Value::String(String::from("nginx"));
    assert_eq!(value, expected);

    let value = nginx.get("image").expect("no image in nginx");
    let expected = &Value::String(String::from("nginx"));
    assert_eq!(value, expected);

    let mounts = nginx.get("volumeMounts").expect("no mounts in nginx");
    let Value::Sequence(mounts) = mounts else {
        panic!("mounts is not sequence");
    };
    assert_eq!(mounts.len(), 1);

    let mount = mounts.get(0).unwrap();
    let value = mount.get("name").expect("no name in mount");
    let expected = &Value::String(String::from("shared-data"));
    assert_eq!(value, expected);

    let value = mount.get("mountPath").expect("no mountPath in mount");
    let expected = &Value::String(String::from("/usr/share/nginx/html"));
    assert_eq!(value, expected);

    // debian container
    let debian = &containers[1];
    let value = debian.get("name").expect("no name in debian");
    let expected = &Value::String(String::from("debian-container"));
    assert_eq!(value, expected);

    let value = debian.get("image").expect("no image in debian");
    let expected = &Value::String(String::from("debian"));
    assert_eq!(value, expected);

    let mounts = debian.get("volumeMounts").expect("no mounts in debian");
    let Value::Sequence(mounts) = mounts else {
        panic!("mounts is not sequence");
    };
    assert_eq!(mounts.len(), 1);

    let mount = &mounts[0];
    let value = mount.get("name").expect("no name in mount");
    let expected = &Value::String(String::from("shared-data"));
    assert_eq!(value, expected);

    let value = mount.get("mountPath").expect("no mountPath in mount");
    let expected = &Value::String(String::from("/pod-data"));
    assert_eq!(value, expected);

    let command = debian.get("command").expect("no command in debian");
    let Value::Sequence(command) = command else {
        panic!("command is not sequence");
    };
    assert_eq!(command.len(), 1);
    let value = &command[0];
    let expected = &Value::String(String::from("/bin/sh"));
    assert_eq!(value, expected);

    let args = debian.get("args").expect("no args in debian");
    let Value::Sequence(args) = args else {
        panic!("args is not sequence");
    };
    assert_eq!(args.len(), 2);

    let expected_args = vec![
        Value::String(String::from("-c")),
        Value::String(String::from(
            "echo Hello from the debian container > /pod-data/index.html",
        )),
    ];

    for (i, value) in args.iter().enumerate() {
        assert_eq!(value, &expected_args[i]);
    }

    Ok(())
}

#[test]
fn multidoc() -> Result<()> {
    let output = Command::cargo_bin("kluars")?
        .args(["xlate", "lua/nginx-app/"])
        .output()?;
    assert!(output.status.success());
    let out = String::from_utf8(output.stdout)?;

    let mut out = serde_yaml::Deserializer::from_str(&out);
    let service = out.next().expect("no service");
    let service: HashMap<String, Value> = deserialize(service)?;
    let deployment = out.next().expect("no deployment");
    let deploy: HashMap<String, Value> = deserialize(deployment)?;
    assert!(out.next().is_none());

    // service
    //
    // kind: Service
    let value = service.get("kind").expect("kind not found");
    let expected = &Value::String(String::from("Service"));
    assert_eq!(value, expected);

    // apiVersion: v1
    let value = service.get("apiVersion").expect("apiVersion not found");
    let expected = &Value::String(String::from("v1"));
    assert_eq!(value, expected);

    let metadata = service.get("metadata").expect("metadata not found");
    let value = metadata.get("name").expect("no name in metadata");
    let expected = &Value::String(String::from("my-nginx-svc"));
    assert_eq!(value, expected);

    let labels = metadata.get("labels").expect("no labels in metadata");
    let value = labels.get("app").expect("no app in labels");
    let expected = &Value::String(String::from("nginx"));
    assert_eq!(value, expected);

    let spec = service.get("spec").expect("spec not found");
    let value = spec.get("type").expect("no type in spec");
    let expected = &Value::String(String::from("LoadBalancer"));
    assert_eq!(value, expected);

    let Value::Sequence(ports) = spec.get("ports").expect("no ports in spec") else {
        panic!("ports is not sequence");
    };
    assert_eq!(ports.len(), 1);
    let port = &ports[0];
    let Value::Number(value) = port.get("port").expect("no port in port") else {
        panic!("port is not number");
    };
    let expected = 80;
    assert_eq!(value.as_u64().unwrap(), expected);

    let selector = spec.get("selector").expect("no selector in spec");
    let value = selector.get("app").expect("no app in selector");
    let expected = &Value::String(String::from("nginx"));
    assert_eq!(value, expected);

    // deployment
    //
    // kind: Deployment
    let value = deploy.get("kind").expect("kind not found");
    let expected = &Value::String(String::from("Deployment"));
    assert_eq!(value, expected);

    // apiVersion: v1
    let value = deploy.get("apiVersion").expect("apiVersion not found");
    let expected = &Value::String(String::from("apps/v1"));
    assert_eq!(value, expected);

    let metadata = deploy.get("metadata").expect("no metadata in deployment");
    let value = metadata.get("name").expect("no name in metadata");
    let expected = &Value::String(String::from("my-nginx"));
    assert_eq!(value, expected);

    let value = metadata
        .get("labels")
        .expect("no labels in metadata")
        .get("app")
        .expect("no app in labels");
    let expected = &Value::String(String::from("nginx"));
    assert_eq!(value, expected);

    let spec = deploy.get("spec").expect("no spec in deployment");
    let Value::Number(value) = spec.get("replicas").expect("no replicas in spec") else {
        panic!("replicas is not number");
    };
    let expected = 3;
    assert_eq!(value.as_u64().unwrap(), expected);

    let Value::String(value) = spec
        .get("selector")
        .expect("no selector in spec")
        .get("matchLabels")
        .expect("no matchLabels in selector")
        .get("app")
        .expect("no app in matchLabels")
    else {
        panic!("app is not string");
    };
    let expected = &String::from("nginx");
    assert_eq!(value, expected);

    let template = spec.get("template").expect("no template in spec");
    let Value::String(value) = template
        .get("metadata")
        .expect("no metadata in template")
        .get("labels")
        .expect("no labels in metadata")
        .get("app")
        .expect("no app in labels")
    else {
        panic!("app is not string");
    };
    let expected = &String::from("nginx");
    assert_eq!(value, expected);

    let Value::Sequence(containers) = template
        .get("spec")
        .expect("no spec in template")
        .get("containers")
        .expect("no container in spec")
    else {
        panic!("container is not sequence");
    };

    assert_eq!(containers.len(), 1);
    let container = &containers[0];
    let Value::String(value) = container.get("name").expect("no name in container") else {
        panic!("name is not string");
    };
    let expected = &String::from("nginx");
    assert_eq!(value, expected);

    let Value::String(value) = container.get("image").expect("no image in container") else {
        panic!("image is not string");
    };
    let expected = &String::from("nginx:1.14.2");
    assert_eq!(value, expected);

    let Value::Sequence(ports) = container.get("ports").expect("no ports in container") else {
        panic!("ports is not sequence");
    };
    assert_eq!(ports.len(), 1);
    let Value::Number(port) = &ports[0]
        .get("containerPort")
        .expect("no containerPort in ports")
    else {
        panic!("port is not number");
    };
    assert_eq!(port.as_u64().unwrap(), 80);

    Ok(())
}
