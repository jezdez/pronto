//! Command-line interface definitions.

use std::path::PathBuf;

use clap::{CommandFactory, FromArgMatches, Parser};

use crate::policy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Normal,
    Verbose,
    Quiet,
}

#[derive(Debug, Parser)]
#[clap(
    disable_help_subcommand = true,
    version,
    allow_external_subcommands = true
)]
pub struct Cli {
    /// Increase output detail
    #[clap(long, short, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[clap(long, short, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

impl Cli {
    pub fn parse_runtime() -> Self {
        Self::parse_runtime_from(std::env::args_os())
    }

    pub fn parse_runtime_from<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let matches = Self::runtime_command().get_matches_from(args);
        Self::from_arg_matches(&matches).unwrap_or_else(|err| err.exit())
    }

    pub fn verbosity(&self) -> Verbosity {
        if self.verbose {
            Verbosity::Verbose
        } else if self.quiet {
            Verbosity::Quiet
        } else {
            Verbosity::Normal
        }
    }

    fn runtime_command() -> clap::Command {
        Self::command()
            .name(policy::command_name())
            .about("Single-binary conda bootstrap runtime")
            .long_about(
                "This runtime bootstraps a conda prefix from a stamped lockfile.\n\n\
                After bootstrap, commands are passed through to the installed conda executable.",
            )
            .after_help(runtime_after_help())
    }
}

fn runtime_after_help() -> String {
    let name = policy::command_name();
    format!(
        "\x1b[1;4mQuick start:\x1b[0m\n\n  \
        {name} bootstrap                           Install conda into ~/{prefix}\n  \
        {name} create -n myenv python=3.12 numpy   Create an environment\n  \
        {name} shell myenv                         Activate (spawns a subshell)\n  \
        exit                                   Leave the environment\n\n\
        \x1b[1;4mManagement:\x1b[0m\n\n  \
        {name} status                              Show installation details\n  \
        {name} uninstall                           Remove conda prefix and all environments\n\n\
        \x1b[1;4mPass-through:\x1b[0m\n\n  \
        Any command not listed above is passed through to conda:\n  \
        {name} install, {name} remove, {name} list, {name} env, {name} info, {name} config, ...\n\n\
        \x1b[4mDocs:\x1b[0m {docs_url}",
        prefix = policy::default_prefix_dir(),
        docs_url = policy::docs_url(),
    )
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Bootstrap a fresh conda installation into the prefix
    Bootstrap {
        /// Force re-bootstrap even if the prefix already exists
        #[clap(long)]
        force: bool,

        /// Target prefix directory (default: distribution-specific home prefix)
        #[clap(long)]
        prefix: Option<PathBuf>,

        /// Channels to use for a live solve (default: stamped runtime channels)
        #[clap(short, long)]
        channel: Option<Vec<String>>,

        /// Additional package specs to install during a live solve
        #[clap(short, long)]
        package: Option<Vec<String>>,

        /// Ignore the embedded lockfile and perform a live solve instead
        #[clap(long)]
        no_lock: bool,

        /// Use an external lockfile instead of the embedded one
        #[clap(long)]
        lockfile: Option<PathBuf>,

        /// Directory containing pre-downloaded .conda / .tar.bz2 package archives
        /// for offline installation
        #[clap(long)]
        bundle: Option<PathBuf>,

        /// Disable network access (install only from cache or --bundle)
        #[clap(long)]
        offline: bool,
    },

    /// Print runtime status (prefix, channels, packages)
    Status {
        /// Target prefix directory (default: distribution-specific home prefix)
        #[clap(long)]
        prefix: Option<PathBuf>,
    },

    /// Activate an environment via subshell (alias for conda spawn)
    Shell {
        /// Name of the environment to activate
        env: Option<String>,
    },

    /// Uninstall this runtime: remove the conda prefix and environments
    Uninstall {
        /// Target prefix directory (default: distribution-specific home prefix)
        #[clap(long)]
        prefix: Option<PathBuf>,

        /// Skip confirmation prompt
        #[clap(long, short)]
        yes: bool,
    },

    /// Show this help message
    Help,
}

pub enum LockSource {
    Embedded,
    File(PathBuf),
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use rstest::rstest;

    #[test]
    fn test_parse_bootstrap_defaults() {
        let cli = Cli::parse_from(["pronto-runtime", "bootstrap"]);
        assert_matches!(
            cli.command,
            Some(Command::Bootstrap {
                force: false,
                prefix: None,
                channel: None,
                package: None,
                no_lock: false,
                lockfile: None,
                bundle: None,
                offline: false,
            })
        );
    }

    #[test]
    fn test_parse_bootstrap_force_prefix() {
        let cli = Cli::parse_from([
            "pronto-runtime",
            "bootstrap",
            "--force",
            "--prefix",
            "/tmp/test",
        ]);
        assert_matches!(
            cli.command,
            Some(Command::Bootstrap { force: true, prefix: Some(ref p), .. })
                if p == std::path::Path::new("/tmp/test")
        );
    }

    #[test]
    fn test_parse_bootstrap_channels_packages() {
        let cli = Cli::parse_from(["pronto-runtime", "bootstrap", "-c", "main", "-p", "numpy"]);
        assert_matches!(
            cli.command,
            Some(Command::Bootstrap { channel: Some(ref c), package: Some(ref p), .. })
                if c == &vec!["main".to_string()] && p == &vec!["numpy".to_string()]
        );
    }

    #[test]
    fn test_parse_bootstrap_multiple_channels_packages() {
        let cli = Cli::parse_from([
            "pronto-runtime",
            "bootstrap",
            "-c",
            "conda-forge",
            "-c",
            "defaults",
            "-p",
            "numpy",
            "-p",
            "scipy",
        ]);
        assert_matches!(
            cli.command,
            Some(Command::Bootstrap { channel: Some(ref c), package: Some(ref p), .. })
                if c == &vec!["conda-forge".to_string(), "defaults".to_string()]
                    && p == &vec!["numpy".to_string(), "scipy".to_string()]
        );
    }

    #[test]
    fn test_parse_bootstrap_no_lock() {
        let cli = Cli::parse_from(["pronto-runtime", "bootstrap", "--no-lock"]);
        assert_matches!(
            cli.command,
            Some(Command::Bootstrap {
                no_lock: true,
                lockfile: None,
                ..
            })
        );
    }

    #[test]
    fn test_parse_bootstrap_lockfile() {
        let cli = Cli::parse_from(["pronto-runtime", "bootstrap", "--lockfile", "/tmp/my.lock"]);
        assert_matches!(
            cli.command,
            Some(Command::Bootstrap { no_lock: false, lockfile: Some(ref p), .. })
                if p == std::path::Path::new("/tmp/my.lock")
        );
    }

    #[test]
    fn test_parse_status() {
        let cli = Cli::parse_from(["pronto-runtime", "status"]);
        assert_matches!(cli.command, Some(Command::Status { prefix: None }));
    }

    #[test]
    fn test_parse_status_with_prefix() {
        let cli = Cli::parse_from([
            "pronto-runtime",
            "status",
            "--prefix",
            "/opt/pronto-runtime",
        ]);
        assert_matches!(
            cli.command,
            Some(Command::Status { prefix: Some(ref p) })
                if p == std::path::Path::new("/opt/pronto-runtime")
        );
    }

    #[test]
    fn test_parse_shell_with_env() {
        let cli = Cli::parse_from(["pronto-runtime", "shell", "myenv"]);
        assert_matches!(
            cli.command,
            Some(Command::Shell { env: Some(ref e) }) if e == "myenv"
        );
    }

    #[test]
    fn test_parse_shell_no_env() {
        let cli = Cli::parse_from(["pronto-runtime", "shell"]);
        assert_matches!(cli.command, Some(Command::Shell { env: None }));
    }

    #[test]
    fn test_parse_uninstall_yes() {
        let cli = Cli::parse_from(["pronto-runtime", "uninstall", "--yes"]);
        assert_matches!(
            cli.command,
            Some(Command::Uninstall {
                yes: true,
                prefix: None
            })
        );
    }

    #[test]
    fn test_parse_uninstall_with_prefix() {
        let cli = Cli::parse_from([
            "pronto-runtime",
            "uninstall",
            "--prefix",
            "/opt/pronto-runtime",
            "-y",
        ]);
        assert_matches!(
            cli.command,
            Some(Command::Uninstall { yes: true, prefix: Some(ref p) })
                if p == std::path::Path::new("/opt/pronto-runtime")
        );
    }

    #[test]
    fn test_parse_no_args() {
        let cli = Cli::parse_from(["pronto-runtime"]);
        assert!(
            cli.command.is_none(),
            "bare `runtime` should have no command"
        );
    }

    #[test]
    fn test_parse_verbose_flag() {
        let cli = Cli::parse_from(["pronto-runtime", "--verbose", "bootstrap"]);
        assert_eq!(cli.verbosity(), Verbosity::Verbose);
    }

    #[test]
    fn test_parse_quiet_flag() {
        let cli = Cli::parse_from(["pronto-runtime", "--quiet", "bootstrap"]);
        assert_eq!(cli.verbosity(), Verbosity::Quiet);
    }

    #[test]
    fn test_parse_short_verbose_flag() {
        let cli = Cli::parse_from(["pronto-runtime", "-v", "status"]);
        assert_eq!(cli.verbosity(), Verbosity::Verbose);
    }

    #[test]
    fn test_parse_short_quiet_flag() {
        let cli = Cli::parse_from(["pronto-runtime", "-q", "status"]);
        assert_eq!(cli.verbosity(), Verbosity::Quiet);
    }

    #[test]
    fn test_parse_no_verbosity_flags() {
        let cli = Cli::parse_from(["pronto-runtime", "bootstrap"]);
        assert_eq!(cli.verbosity(), Verbosity::Normal);
    }

    #[test]
    fn test_verbose_quiet_conflict() {
        let result = Cli::try_parse_from(["pronto-runtime", "--verbose", "--quiet", "bootstrap"]);
        assert!(result.is_err(), "--verbose and --quiet should conflict");
    }

    #[rstest]
    #[case::bundle_only(&["pronto-runtime", "bootstrap", "--bundle", "/tmp/pkgs"], Some("/tmp/pkgs"), false)]
    #[case::offline_only(&["pronto-runtime", "bootstrap", "--offline"], None, true)]
    #[case::both(&["pronto-runtime", "bootstrap", "--bundle", "/p", "--offline"], Some("/p"), true)]
    #[case::defaults(&["pronto-runtime", "bootstrap"], None, false)]
    fn test_parse_bootstrap_offline_flags(
        #[case] args: &[&str],
        #[case] expected_bundle: Option<&str>,
        #[case] expected_offline: bool,
    ) {
        let cli = Cli::parse_from(args);
        match cli.command {
            Some(Command::Bootstrap {
                bundle, offline, ..
            }) => {
                assert_eq!(bundle.as_deref(), expected_bundle.map(std::path::Path::new));
                assert_eq!(offline, expected_offline);
            }
            other => panic!("expected Bootstrap, got {other:?}"),
        }
    }
}
