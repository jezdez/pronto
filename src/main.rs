//! Generic single-binary conda bootstrap runtime powered by rattler.

use std::env;

use miette::IntoDiagnostic;

mod cli;
mod commands;
mod config;
mod exec;
mod install;
mod policy;
mod runtime_data;

use cli::{Cli, Command, LockSource};
use commands::{
    bootstrap, ensure_bootstrapped, is_bootstrapped, print_disabled_init,
    print_disabled_shell_command, status, uninstall, validate_bootstrap_flags,
};

fn main() -> miette::Result<()> {
    let num_cores = std::thread::available_parallelism()
        .map_or(2, std::num::NonZero::get)
        .max(2);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cores / 2)
        .max_blocking_threads(num_cores)
        .enable_all()
        .build()
        .into_diagnostic()?;

    runtime.block_on(async_main())
}

async fn async_main() -> miette::Result<()> {
    init_tracing()?;

    let raw_args: Vec<String> = env::args().collect();
    let first_arg = raw_args.get(1).map(|s| s.as_str());

    match first_arg {
        Some("activate") | Some("deactivate") => {
            print_disabled_shell_command(first_arg.unwrap());
        }
        Some("init") => {
            print_disabled_init();
        }
        Some("bootstrap") | Some("status") | Some("shell") | Some("uninstall") | Some("help")
        | Some("--help") | Some("-h") | Some("--version") | Some("-V") | None => {
            let cli = Cli::parse_runtime();
            let verbosity = cli.verbosity();
            match cli.command {
                Some(Command::Bootstrap {
                    force,
                    prefix,
                    channel,
                    package,
                    no_lock,
                    lockfile,
                    bundle,
                    offline,
                }) => {
                    let prefix = prefix.map(Ok).unwrap_or_else(policy::default_prefix)?;
                    let bundle = bundle.or_else(|| {
                        env::var(policy::bundle_env_var())
                            .ok()
                            .filter(|v| !v.is_empty())
                            .map(std::path::PathBuf::from)
                    });
                    let offline = offline
                        || env::var(policy::offline_env_var())
                            .ok()
                            .filter(|v| !v.is_empty())
                            .is_some_and(|v| v != "0" && v.to_lowercase() != "false");

                    validate_bootstrap_flags(
                        offline, no_lock, &lockfile, &bundle, &channel, &package,
                    )?;

                    let lock_source = if no_lock {
                        LockSource::None
                    } else if let Some(path) = lockfile {
                        LockSource::File(path)
                    } else {
                        LockSource::Embedded
                    };

                    return bootstrap(
                        &prefix,
                        force,
                        channel,
                        package,
                        lock_source,
                        bundle,
                        offline,
                        verbosity,
                    )
                    .await;
                }
                Some(Command::Status { prefix }) => {
                    let prefix = prefix.map(Ok).unwrap_or_else(policy::default_prefix)?;
                    return status(&prefix);
                }
                Some(Command::Uninstall { prefix, yes }) => {
                    let prefix = prefix.map(Ok).unwrap_or_else(policy::default_prefix)?;
                    return uninstall(&prefix, yes, verbosity);
                }
                Some(Command::Shell { env }) => {
                    let prefix = policy::default_prefix()?;
                    ensure_bootstrapped(&prefix).await?;
                    let mut conda_args = vec!["spawn"];
                    if let Some(ref name) = env {
                        conda_args.push(name);
                    }
                    let extra: Vec<&str> = raw_args[2..]
                        .iter()
                        .skip(env.is_some() as usize)
                        .map(|s| s.as_str())
                        .collect();
                    conda_args.extend(extra);
                    return exec::replace_process_with_conda(&prefix, &conda_args);
                }
                Some(Command::Help) => {
                    Cli::parse_runtime_from([policy::command_name(), "--help"]);
                }
                None => {
                    let prefix = policy::default_prefix()?;
                    if !is_bootstrapped(&prefix) {
                        eprintln!(
                            "{} No conda installation found. Run `{} bootstrap` first.",
                            console::style("!").yellow().bold(),
                            policy::command_name()
                        );
                        std::process::exit(1);
                    }
                    return exec::replace_process_with_conda(&prefix, &["--help"]);
                }
            }
        }
        Some(_) => {
            let prefix = policy::default_prefix()?;
            ensure_bootstrapped(&prefix).await?;
            let conda_args: Vec<&str> = raw_args[1..].iter().map(|s| s.as_str()).collect();
            if exec::should_filter_conda_output(&conda_args) {
                return exec::run_conda_filtered(&prefix, &conda_args);
            }
            return exec::replace_process_with_conda(&prefix, &conda_args);
        }
    }
    Ok(())
}

fn init_tracing() -> miette::Result<()> {
    use tracing_subscriber::{EnvFilter, filter::LevelFilter, util::SubscriberInitExt};

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::WARN.into())
        .from_env()
        .into_diagnostic()?;

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .without_time()
        .finish()
        .try_init()
        .into_diagnostic()?;

    Ok(())
}
