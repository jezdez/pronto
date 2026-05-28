//! Runtime distribution policy.
//!
//! The install/runtime code is generic. Values in this module describe the
//! concrete `cx` distribution built from it.

use std::path::PathBuf;

pub(crate) const COMMAND_NAME: &str = "cx";
pub(crate) const EMBEDDED_COMMAND_NAME: &str = "cxz";
pub(crate) const DISPLAY_NAME: &str = "cx";
pub(crate) const DEFAULT_PREFIX_DIR: &str = ".cx";
pub(crate) const METADATA_FILE: &str = ".cx.json";
pub(crate) const BUNDLE_ENV_VAR: &str = "CX_BUNDLE";
pub(crate) const OFFLINE_ENV_VAR: &str = "CX_OFFLINE";

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
