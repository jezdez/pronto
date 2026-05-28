//! Runtime distribution policy.
//!
//! The install/runtime code is generic. Values in this module are embedded by
//! the builder for each generated distribution.

use std::path::PathBuf;

pub(crate) const COMMAND_NAME: &str = env!("PRONTO_RUNTIME_NAME");
pub(crate) const EMBEDDED_COMMAND_NAME: &str = env!("PRONTO_RUNTIME_EMBEDDED_NAME");
pub(crate) const DISPLAY_NAME: &str = env!("PRONTO_RUNTIME_DISPLAY_NAME");
pub(crate) const DEFAULT_PREFIX_DIR: &str = env!("PRONTO_RUNTIME_PREFIX_DIR");
pub(crate) const METADATA_FILE: &str = env!("PRONTO_RUNTIME_METADATA_FILE");
pub(crate) const BUNDLE_ENV_VAR: &str = env!("PRONTO_RUNTIME_BUNDLE_ENV_VAR");
pub(crate) const OFFLINE_ENV_VAR: &str = env!("PRONTO_RUNTIME_OFFLINE_ENV_VAR");

pub(crate) fn default_prefix() -> miette::Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| miette::miette!("could not determine home directory"))?;
    Ok(home.join(DEFAULT_PREFIX_DIR))
}

pub(crate) fn status_binary_name(embedded_bundle: &[u8]) -> &'static str {
    if embedded_bundle.is_empty() {
        COMMAND_NAME
    } else {
        EMBEDDED_COMMAND_NAME
    }
}

pub(crate) fn frozen_message() -> String {
    format!(
        "This base environment is managed by {display}.\n\
Create a new environment instead: conda create -n myenv\n\
To re-bootstrap: {command} bootstrap --force\n\
To override: pass --override-frozen-env",
        display = DISPLAY_NAME,
        command = COMMAND_NAME
    )
}
