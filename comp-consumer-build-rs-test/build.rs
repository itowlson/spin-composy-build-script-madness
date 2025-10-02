use anyhow::Context;
use std::path::{Path, PathBuf};

use wit_component::DecodedWasm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=spin.toml");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // std::fs::write("./out.txt", format!("{}", out_dir.display())).unwrap();
    let s = std::fs::read_to_string("./spin.toml").unwrap();
    let t: toml::Table = toml::from_str(&s).unwrap();
    let c = t.get("component").unwrap().get("comp-consumer-build-rs-test").unwrap();
    if let Some(deps) = c.get("dependencies").and_then(|d| d.as_table()) {
        let dest_path = out_dir.join(format!("biscuits.rs"));

        let mut macks = String::new();

        for (depname, dep) in deps {
            if let Some(deppath) = dep.get("path").and_then(|p| p.as_str()) {
                println!("cargo::rerun-if-changed={deppath}");

                let enc_wit_path = out_dir.join(format!("{}.wasm", safeify(depname)));

                let mut wasm = read_wasm(deppath)?;
                importize(&mut wasm, None, None)?;
                emit_wasm(&wasm, &enc_wit_path)?;

                let itfs = extract_imports(wasm);

                let ns = &itfs[0].0.namespace;

                let mack = build_macro_call(&enc_wit_path, &itfs, ns);

                macks = format!("{macks}\n{mack}");
            }
        }

        std::fs::write(&dest_path, macks).unwrap();
    }

    Ok(())
}

fn extract_imports(wasm: DecodedWasm) -> Vec<(wit_parser::PackageName, String)> {
    let mut itfs = vec![];

    for (_pid, pp) in &wasm.resolve().packages {
        for (_w, wid) in &pp.worlds {
            if let Some(world) = wasm.resolve().worlds.get(*wid) {
                for (_wk, witem) in &world.imports {
                    if let wit_parser::WorldItem::Interface { id, .. } = witem {
                        if let Some(itf) = wasm.resolve().interfaces.get(*id) {
                            if let Some(itfp) = itf.package.as_ref() {
                                if let Some(ppp) = wasm.resolve().packages.get(*itfp) {
                                    if let Some(itfname) = itf.name.as_ref() {
                                        itfs.push((ppp.name.clone(), itfname.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    itfs
}

fn read_wasm(path: impl AsRef<Path>) -> anyhow::Result<DecodedWasm> {
    let wasm_bytes = std::fs::read(path.as_ref())?;

    if wasmparser::Parser::is_component(&wasm_bytes) {
        wit_component::decode(&wasm_bytes)
    } else {
        let (wasm, bindgen) = wit_component::metadata::decode(&wasm_bytes)?;
        if wasm.is_none() {
            anyhow::bail!(
                "input is a core wasm module with no `component-type*` \
                    custom sections meaning that there is not WIT information; \
                    is the information not embedded or is this supposed \
                    to be a component?"
            )
        }
        Ok(DecodedWasm::Component(bindgen.resolve, bindgen.world))
    }
}

fn importize(
    decoded: &mut DecodedWasm,
    world: Option<&str>,
    out_world_name: Option<&String>,
) -> anyhow::Result<()> {
    let (resolve, world_id) = match (&mut *decoded, world) {
        (DecodedWasm::Component(resolve, world), None) => (resolve, *world),
        (DecodedWasm::Component(..), Some(_)) => {
            anyhow::bail!(
                "the `--importize-world` flag is not compatible with a \
                    component input, use `--importize` instead"
            );
        }
        (DecodedWasm::WitPackage(resolve, id), world) => {
            let world = resolve.select_world(&[*id], world)?;
            (resolve, world)
        }
    };
    resolve
        .importize(world_id, out_world_name.cloned())
        .context("failed to move world exports to imports")?;
    let resolve = std::mem::take(resolve);
    *decoded = DecodedWasm::Component(resolve, world_id);
    Ok(())
}

fn emit_wasm(decoded: &DecodedWasm, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let decoded_package = decoded.package();
    let bytes = wit_component::encode(decoded.resolve(), decoded_package)?;
    std::fs::write(dest.as_ref(), bytes)?;
    Ok(())
}

fn build_macro_call(wasm_path: impl AsRef<Path>, itfs: &[(wit_parser::PackageName, String)], ns: &str) -> String {
    let import_clauses = itfs.iter().map(|(p, i)| format!("import {};", qname(p, i))).collect::<Vec<_>>();
    format!(r##"mod {ns}_dep {{
spin_sdk::wit_bindgen::generate!({{
    inline: r#"
    package test:test-{ns};
    world spork {{
        {}
    }}
    "#,
    path: "{}",
    world: "test:test-{ns}/spork",
    runtime_path: "::spin_sdk::wit_bindgen::rt",
    generate_all
}});
}}
"##, import_clauses.join("\n"), wasm_path.as_ref().display())
}

fn qname(p: &wit_parser::PackageName, i: &str) -> String {
    match &p.version {
        Some(v) => format!("{}:{}/{}@{}", p.namespace, p.name, i, v),
        None => format!("{}:{}/{}", p.namespace, p.name, i),
    }
}

fn safeify(depname: &str) -> String {
    depname.replace("/", "_").replace("@", "_").replace(":", "_").replace(".", "_")
}
