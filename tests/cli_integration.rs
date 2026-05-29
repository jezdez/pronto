//! Integration tests for the pronto-runtime binary using assert_cmd.
#![cfg(feature = "runtime-template")]

use std::path::PathBuf;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use rstest::rstest;
use tempfile::TempDir;

fn runtime() -> assert_cmd::Command {
    cargo_bin_cmd!("pronto-runtime")
}

#[test]
fn test_runtime_help() {
    let output = runtime().arg("--help").output().unwrap();
    assert!(
        output.status.success(),
        "pronto-runtime --help should succeed"
    );
    let stdout =
        String::from_utf8_lossy(&output.stdout).replace("pronto-runtime.exe", "pronto-runtime");
    insta::assert_snapshot!("runtime_help", stdout);
}

#[test]
fn test_runtime_version() {
    runtime()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[derive(Clone, Copy)]
enum MissingPrefixCmd {
    Status,
    Uninstall,
}

#[rstest]
#[case::status(MissingPrefixCmd::Status)]
#[case::uninstall(MissingPrefixCmd::Uninstall)]
fn test_runtime_nonexistent_prefix_reports_missing(#[case] cmd: MissingPrefixCmd) {
    let tmp = TempDir::new().unwrap();
    let nonexistent = tmp.path().join("does-not-exist");
    let path = nonexistent.to_str().unwrap();

    let mut c = runtime();
    match cmd {
        MissingPrefixCmd::Status => {
            c.args(["status", "--prefix", path]);
        }
        MissingPrefixCmd::Uninstall => {
            c.args(["uninstall", "--prefix", path, "--yes"]);
        }
    }
    c.assert()
        .success()
        .stderr(predicate::str::contains("No conda installation found"));
}

#[test]
fn test_runtime_bootstrap_already_exists() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir(tmp.path().join("conda-meta")).unwrap();

    runtime()
        .args(["bootstrap", "--prefix", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("already bootstrapped"));
}

#[cfg_attr(not(feature = "online_tests"), ignore)]
#[test]
fn test_runtime_bootstrap_to_temp_prefix() {
    let tmp = TempDir::new().unwrap();
    let prefix = tmp.path().join("pronto-runtime-test-bootstrap");

    runtime()
        .args(["bootstrap", "--prefix", prefix.to_str().unwrap()])
        .timeout(std::time::Duration::from_secs(120))
        .assert()
        .success()
        .stderr(predicate::str::contains("bootstrapped successfully"));

    assert!(
        prefix.join("conda-meta").is_dir(),
        "conda-meta should exist"
    );
    assert!(
        prefix.join(".pronto-runtime.json").exists(),
        ".pronto-runtime.json should exist"
    );
    assert!(prefix.join(".condarc").exists(), ".condarc should exist");
    assert!(
        prefix.join("conda-meta/frozen").exists(),
        "frozen marker should exist"
    );
}

#[cfg_attr(not(feature = "online_tests"), ignore)]
#[test]
fn test_runtime_status_after_bootstrap() {
    let tmp = TempDir::new().unwrap();
    let prefix = tmp.path().join("pronto-runtime-test-status");

    runtime()
        .args(["bootstrap", "--prefix", prefix.to_str().unwrap()])
        .timeout(std::time::Duration::from_secs(120))
        .assert()
        .success();

    runtime()
        .args(["status", "--prefix", prefix.to_str().unwrap()])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("channels:")
                .and(predicate::str::contains("packages:"))
                .and(predicate::str::contains("installed:")),
        );
}

#[cfg_attr(not(feature = "online_tests"), ignore)]
#[test]
fn test_runtime_uninstall_removes_prefix() {
    let tmp = TempDir::new().unwrap();
    let prefix = tmp.path().join("pronto-runtime-test-uninstall");

    runtime()
        .args(["bootstrap", "--prefix", prefix.to_str().unwrap()])
        .timeout(std::time::Duration::from_secs(120))
        .assert()
        .success();

    assert!(prefix.exists(), "prefix should exist after bootstrap");

    runtime()
        .args(["uninstall", "--prefix", prefix.to_str().unwrap(), "--yes"])
        .assert()
        .success()
        .stderr(predicate::str::contains("uninstalled"));

    assert!(!prefix.exists(), "prefix should be removed after uninstall");
}

/// Uses `--prefix` so Windows CI does not depend on `dirs::home_dir()` (known-folder profile),
/// which ignores `HOME` / `USERPROFILE` for a synthetic layout.
#[test]
fn test_runtime_uninstall_interactive_prompt_declined() {
    let tmp = TempDir::new().unwrap();
    let prefix = tmp.path().join("pronto-runtime-uninstall-interactive");
    std::fs::create_dir_all(prefix.join("conda-meta")).unwrap();

    runtime()
        .args(["uninstall", "--prefix", prefix.to_str().unwrap()])
        .write_stdin("n\n")
        .assert()
        .success()
        .stderr(
            predicate::str::contains("Continue? [y/N]").and(predicate::str::contains("Aborted.")),
        );

    assert!(
        prefix.exists(),
        "prefix should remain after declining uninstall"
    );
}

#[test]
fn test_runtime_bootstrap_offline_no_lock_rejected() {
    let tmp = TempDir::new().unwrap();
    runtime()
        .args([
            "bootstrap",
            "--prefix",
            tmp.path().to_str().unwrap(),
            "--offline",
            "--no-lock",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("incompatible"));
}

#[test]
fn test_runtime_bootstrap_package_requires_live_solve() {
    let tmp = TempDir::new().unwrap();
    runtime()
        .args([
            "bootstrap",
            "--prefix",
            tmp.path().to_str().unwrap(),
            "--package",
            "numpy",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("only affect live solves"));
}

#[test]
fn test_runtime_bootstrap_bundle_bad_dir_rejected() {
    let tmp = TempDir::new().unwrap();
    let bad_dir = tmp.path().join("nonexistent");
    runtime()
        .args([
            "bootstrap",
            "--prefix",
            tmp.path().to_str().unwrap(),
            "--bundle",
            bad_dir.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a directory"));
}

#[cfg_attr(not(feature = "online_tests"), ignore)]
#[test]
fn test_runtime_bootstrap_offline_from_cache() {
    let tmp = TempDir::new().unwrap();
    let prefix1 = tmp.path().join("online");
    let prefix2 = tmp.path().join("offline");

    runtime()
        .args(["bootstrap", "--prefix", prefix1.to_str().unwrap()])
        .timeout(std::time::Duration::from_secs(120))
        .assert()
        .success();

    runtime()
        .args([
            "bootstrap",
            "--prefix",
            prefix2.to_str().unwrap(),
            "--offline",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("bootstrapped successfully"));

    assert!(prefix2.join("conda-meta").is_dir());
    assert!(prefix2.join(".pronto-runtime.json").exists());
}

#[cfg_attr(not(feature = "online_tests"), ignore)]
#[test]
fn test_runtime_bootstrap_bundle_offline() {
    let tmp = TempDir::new().unwrap();
    let prefix1 = tmp.path().join("seed");
    let prefix2 = tmp.path().join("offline");
    let bundle_dir = tmp.path().join("bundle");

    runtime()
        .args(["bootstrap", "--prefix", prefix1.to_str().unwrap()])
        .timeout(std::time::Duration::from_secs(120))
        .assert()
        .success();

    let cache_pkgs = rattler_pkgs_cache_dir();
    std::fs::create_dir(&bundle_dir).unwrap();
    if cache_pkgs.is_dir() {
        for entry in std::fs::read_dir(&cache_pkgs).unwrap().flatten() {
            let path = entry.path();
            if path.is_file()
                && (path.extension().is_some_and(|e| e == "conda")
                    || path.to_str().is_some_and(|s| s.ends_with(".tar.bz2")))
            {
                std::fs::copy(&path, bundle_dir.join(path.file_name().unwrap())).unwrap();
            }
        }
    }

    runtime()
        .args([
            "bootstrap",
            "--prefix",
            prefix2.to_str().unwrap(),
            "--bundle",
            bundle_dir.to_str().unwrap(),
            "--offline",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("bootstrapped successfully"));

    assert!(prefix2.join("conda-meta").is_dir());
    assert!(prefix2.join(".pronto-runtime.json").exists());
}

#[rstest]
#[case::set_to_1("1", true)]
#[case::set_to_true("true", true)]
#[case::set_to_yes("yes", true)]
#[case::set_to_0("0", false)]
#[case::set_to_false("false", false)]
#[case::set_to_false_upper("FALSE", false)]
#[case::empty("", false)]
fn test_runtime_offline_env_var_parsing(#[case] value: &str, #[case] expect_offline: bool) {
    let tmp = TempDir::new().unwrap();
    // Pre-create conda-meta so non-offline cases short-circuit to "already bootstrapped"
    // instead of attempting a real solve that requires network.
    std::fs::create_dir(tmp.path().join("conda-meta")).unwrap();

    let mut cmd = runtime();
    cmd.env("PRONTO_RUNTIME_OFFLINE", value).args([
        "bootstrap",
        "--prefix",
        tmp.path().to_str().unwrap(),
        "--no-lock",
    ]);

    if expect_offline {
        // offline + --no-lock is rejected by validate_bootstrap_flags
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("incompatible"));
    } else {
        // offline=false → validation passes → hits "already bootstrapped"
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("already bootstrapped"));
    }
}

#[test]
fn test_runtime_bundle_env_var() {
    let tmp = TempDir::new().unwrap();
    let bad_dir = tmp.path().join("nonexistent");
    runtime()
        .env("PRONTO_RUNTIME_BUNDLE", bad_dir.as_os_str())
        .args(["bootstrap", "--prefix", tmp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a directory"));
}

#[test]
fn test_runtime_status_shows_binary_name_and_version() {
    let tmp = TempDir::new().unwrap();
    let prefix = tmp.path().join("status-name");
    std::fs::create_dir_all(prefix.join("conda-meta")).unwrap();

    let version = env!("CARGO_PKG_VERSION");
    runtime()
        .args(["status", "--prefix", prefix.to_str().unwrap()])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!("pronto-runtime {version}")).or(
                predicate::str::starts_with(format!("pronto-runtimez {version}")),
            ),
        );
}

fn rattler_pkgs_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .expect("cache dir")
        .join("rattler")
        .join("cache")
        .join("pkgs")
}

#[cfg(unix)]
#[test]
fn test_runtime_uninstall_default_prefix_respects_home() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".pronto-runtime/conda-meta")).unwrap();

    runtime()
        .env("HOME", tmp.path().as_os_str())
        .arg("uninstall")
        .write_stdin("n\n")
        .assert()
        .success()
        .stderr(
            predicate::str::contains("Continue? [y/N]").and(predicate::str::contains("Aborted.")),
        );

    assert!(tmp.path().join(".pronto-runtime").exists());
}
