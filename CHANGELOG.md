# Changelog

All notable user-facing changes to `conda-pronto` are documented here.

## 0.1.0 - 2026-05-29

Initial release of `conda-pronto`, a generic builder for ready-to-run conda
bootstrap binaries. The Rust crate is published as `conda-pronto` and installs
the `pronto` CLI. The Python package provides the optional `conda pronto`
subcommand for conda installations that want plugin-style integration.

### Added

- `pronto`, a CLI for building named downstream conda bootstrap binaries from
  committed project metadata and lockfiles.
- `pronto-runtime-template`, the generic runtime template used to produce
  downstream binaries.
- A GitHub Action and reusable workflow for release builds that consume
  published `conda-pronto` assets instead of rebuilding the builder from source.
- Support for `conda.toml` with `conda.lock`, `pixi.toml` with `pixi.lock`, and
  Pixi configuration in `pyproject.toml` with `pixi.lock`.
- Offline bundle generation for runtimes that should install from embedded or
  pre-downloaded conda package archives.
- Package exclusion after lockfile resolution, so downstream distributions can
  trim packages from a solved environment before building a runtime.
- Staged build metadata for generated runtimes, including `.runtime.lock`,
  `.packages.txt`, `.info.json`, and `.sha256` files.
- Release assets for tagged builds: `pronto`, `pronto-runtime-template`, and
  `SHA256SUMS`.
- Crates.io and PyPI packaging metadata for publishing `conda-pronto`.

### Changed

- `conda-pronto` no longer behaves like a downstream distribution. Downstream
  projects choose their own binary name, package set, channels, documentation
  URL, and release channel.
- `pronto build --layout online` builds a runtime that downloads packages
  during bootstrap.
- Generated runtime `.condarc` files now use the channels stamped into the
  runtime instead of assuming `conda-forge`.
- Runtime `--channel` and `--package` flags are now accepted only for live
  solves with `--no-lock`; lockfile-based builds use the committed lockfile
  contents.
- Default builds use platform-native TLS. The `rustls-tls` feature remains
  available for downstream builds that want Rustls explicitly.
- The `conda pronto` adapter now prefers the `pronto` executable installed next
  to the current Python interpreter before falling back to `PATH`.
- The GitHub Action now expects committed manifest and lockfile input. It no
  longer creates or mutates project manifests during CI.

### Security

- Bundle builds now require SHA256 metadata in runtime locks.
- Cached, downloaded, embedded, and offline package archives are verified before
  they are staged or installed.
- The GitHub Action verifies artifact attestations for downloaded `pronto`,
  `pronto-runtime-template`, and `SHA256SUMS` assets before running them.
- GitHub workflows and the composite action use pinned actions, minimal
  permissions, explicit artifact verification, and no shell `eval` for user
  inputs.
- Rust advisory, license, dependency-ban, and source policies are enforced with
  `cargo deny`.

### Removed

- Removed inherited `conda-express` distribution defaults from the generic
  builder. `conda-pronto` does not ship a default downstream runtime binary.
- Removed CI behavior that generated temporary Pixi manifests from action
  inputs.
