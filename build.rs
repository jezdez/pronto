//! Minimal build script that copies generated runtime inputs into `$OUT_DIR`
//! for embedding into named runtime binaries.
//!
//! Pronto derives `target/pronto/runtime.lock` from pixi.lock and filters.
//! `pronto build` passes that path through `PRONTO_RUNTIME_LOCK`. When
//! `PRONTO_EMBED_BUNDLE=1`, it also passes `PRONTO_BUNDLE`.

use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-env-changed=PRONTO_EMBED_BUNDLE");
    println!("cargo:rerun-if-env-changed=PRONTO_BUNDLE");
    println!("cargo:rerun-if-env-changed=PRONTO_DOCS_URL");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_NAME");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_EMBEDDED_NAME");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_DISPLAY_NAME");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_PREFIX_DIR");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_METADATA_FILE");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_BUNDLE_ENV_VAR");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_OFFLINE_ENV_VAR");
    println!("cargo:rerun-if-env-changed=PRONTO_RUNTIME_LOCK");
    println!("cargo:rerun-if-env-changed=PRONTO_INSTALL_METHOD");

    let runtime_name = env::var("PRONTO_RUNTIME_NAME").unwrap_or_else(|_| "pronto-runtime".into());
    let embedded_name =
        env::var("PRONTO_RUNTIME_EMBEDDED_NAME").unwrap_or_else(|_| format!("{runtime_name}z"));
    let display_name =
        env::var("PRONTO_RUNTIME_DISPLAY_NAME").unwrap_or_else(|_| runtime_name.clone());
    let prefix_dir =
        env::var("PRONTO_RUNTIME_PREFIX_DIR").unwrap_or_else(|_| format!(".{runtime_name}"));
    let metadata_file = env::var("PRONTO_RUNTIME_METADATA_FILE")
        .unwrap_or_else(|_| format!(".{runtime_name}.json"));
    let env_prefix = env_var_prefix(&runtime_name);

    set_runtime_env("PRONTO_RUNTIME_NAME", &runtime_name);
    set_runtime_env("PRONTO_RUNTIME_EMBEDDED_NAME", &embedded_name);
    set_runtime_env("PRONTO_RUNTIME_DISPLAY_NAME", &display_name);
    set_runtime_env("PRONTO_RUNTIME_PREFIX_DIR", &prefix_dir);
    set_runtime_env("PRONTO_RUNTIME_METADATA_FILE", &metadata_file);
    set_runtime_env(
        "PRONTO_RUNTIME_BUNDLE_ENV_VAR",
        &env::var("PRONTO_RUNTIME_BUNDLE_ENV_VAR")
            .unwrap_or_else(|_| format!("{env_prefix}_BUNDLE")),
    );
    set_runtime_env(
        "PRONTO_RUNTIME_OFFLINE_ENV_VAR",
        &env::var("PRONTO_RUNTIME_OFFLINE_ENV_VAR")
            .unwrap_or_else(|_| format!("{env_prefix}_OFFLINE")),
    );
    println!(
        "cargo:rustc-env=PRONTO_DOCS_URL={}",
        env::var("PRONTO_DOCS_URL").unwrap_or_else(|_| "https://jezdez.github.io/pronto/".into())
    );

    let runtime_template = env::var_os("CARGO_FEATURE_RUNTIME_TEMPLATE").is_some();
    let runtime_lock_src = env::var_os("PRONTO_RUNTIME_LOCK")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join("target/pronto/runtime.lock"));
    let runtime_lock_dst = out_dir.join("runtime.lock");
    println!("cargo:rerun-if-changed={}", runtime_lock_src.display());
    if runtime_template {
        assert!(
            runtime_lock_src.exists(),
            "runtime lock not found at {} — run `cargo run -p pronto -- lock` first",
            runtime_lock_src.display()
        );
        std::fs::copy(&runtime_lock_src, &runtime_lock_dst)
            .expect("failed to copy runtime.lock to OUT_DIR");
    } else if !runtime_lock_dst.exists() {
        std::fs::write(&runtime_lock_dst, b"").expect("failed to write empty runtime.lock");
    }

    let bundle_dst = out_dir.join("bundle.tar.zst");
    let embed_bundle = env_flag("PRONTO_EMBED_BUNDLE");

    if embed_bundle {
        let bundle_src = env::var_os("PRONTO_BUNDLE")
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("target/pronto/bundle.tar.zst"));
        println!("cargo:rerun-if-changed={}", bundle_src.display());
        assert!(
            bundle_src.exists(),
            "PRONTO_EMBED_BUNDLE=1 but bundle not found at {} — \
             run `cargo run -p pronto -- bundle` first",
            bundle_src.display()
        );
        std::fs::copy(&bundle_src, &bundle_dst).expect("failed to copy bundle.tar.zst");
    } else if !bundle_dst.exists() {
        std::fs::write(&bundle_dst, b"").expect("failed to write empty bundle.tar.zst");
    }
}

fn env_flag(name: &str) -> bool {
    env::var(name)
        .ok()
        .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

fn set_runtime_env(name: &str, value: &str) {
    println!("cargo:rustc-env={name}={value}");
}

fn env_var_prefix(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}
