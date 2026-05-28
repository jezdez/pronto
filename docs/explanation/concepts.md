# Concepts

Pronto separates three concerns:

- resolving and recording a conda runtime package set
- compiling a generic bootstrap runtime with distribution metadata
- staging release artifacts that downstream projects can distribute

## Builder

The `pronto` CLI is the builder. It reads `pixi.toml`, `pixi.lock`, and
`[tool.pronto]`, then derives a runtime lock, bundle files, runtime binaries,
and artifact metadata.

## Runtime Template

`pronto-runtime` is an internal generic binary target. It is not a first-party
distribution. During `pronto build`, the builder compiles this template with the
downstream distribution name, prefix, metadata filename, and environment
variable names embedded into the binary.

## Runtime Lock

The runtime lock is derived from Pixi's `runtime` environment, then filtered
through `[tool.pronto].exclude`. Pronto writes it to `target/pronto/runtime.lock`
as a generated build input and stages a copy next to every output binary. It is
not a second checked-in project lockfile.

The generated runtime can install from:

- the embedded lockfile and network package downloads
- an external lockfile passed with `--lockfile`
- a live solve when `--no-lock` is used

## Bundles

Bundles contain downloaded conda package archives.

The `external` layout pairs a runtime binary with `NAME.bundle.tar.zst`. The
`embedded` layout appends `z` to the binary name and includes the compressed
bundle inside the executable.
