//! Integration tests verifying the embedded runtime lock has been pre-filtered.
#![cfg(feature = "runtime-template")]

use std::path::PathBuf;

use rattler_conda_types::{Platform, RepoDataRecord};
use rattler_lock::LockFile;

fn records_from_embedded_lock() -> Vec<RepoDataRecord> {
    let lock_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("pronto")
        .join("runtime.lock");
    let lock_content = std::fs::read_to_string(&lock_path).unwrap_or_else(|err| {
        panic!(
            "failed to read {}; run `pronto lock` first: {err}",
            lock_path.display()
        )
    });
    let lock_file = LockFile::from_str_with_base_directory(&lock_content, lock_path.parent())
        .expect("failed to parse generated runtime lock");
    let env = lock_file
        .default_environment()
        .expect("no default environment");
    let platform = Platform::current();
    let lock_platform = env
        .platforms()
        .find(|locked_platform| locked_platform.subdir() == platform)
        .unwrap_or_else(|| panic!("no records for current platform {platform}"));
    env.conda_repodata_records(lock_platform)
        .expect("failed to extract records")
        .expect("no records for current platform")
}

fn sorted_names(records: &[RepoDataRecord]) -> Vec<String> {
    let mut names: Vec<String> = records
        .iter()
        .map(|r| r.package_record.name.as_normalized().to_string())
        .collect();
    names.sort();
    names
}

#[test]
fn test_embedded_lockfile_package_composition() {
    let records = records_from_embedded_lock();
    let names = sorted_names(&records);

    let excluded = ["conda-libmamba-solver", "libmamba", "libsolv"];
    for pkg in &excluded {
        assert!(
            !names.contains(&pkg.to_string()),
            "embedded runtime lock should not contain {pkg}"
        );
    }

    let required = ["conda", "conda-rattler-solver", "conda-spawn"];
    for pkg in &required {
        assert!(
            names.contains(&pkg.to_string()),
            "embedded runtime lock should contain {pkg}"
        );
    }
    assert!(
        names.iter().any(|n| n.starts_with("python")),
        "embedded runtime lock should contain python"
    );
}
