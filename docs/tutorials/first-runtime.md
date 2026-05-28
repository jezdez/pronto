# Build Your First Runtime

This tutorial builds a local conda bootstrap binary named `myconda` and runs it
against a temporary prefix.

## Prerequisites

Work from a checkout of the Pronto repository with Pixi available:

```bash
pixi run prepare
```

`prepare` derives the runtime lock from the `runtime` Pixi environment and the
Pronto runtime configuration, then writes it to `target/pronto/runtime.lock`.

## Inspect The Runtime Package Set

Check what will be embedded into the artifact metadata:

```bash
pixi run -- cargo run -p pronto -- inspect
```

The output lists every platform in the derived runtime lock, then prints the
packages for the current platform.

## Build A Network Bootstrap Binary

Build a binary that contains lockfile metadata but downloads package archives
during bootstrap:

```bash
pixi run -- cargo run -p pronto -- build --layout none --name myconda
```

The staged files are written to `dist/`. The binary is named `myconda` on Unix
and `myconda.exe` on Windows.

## Smoke Test The Runtime

Run the staged binary through Pronto:

```bash
pixi run -- cargo run -p pronto -- run --name myconda -- bootstrap --prefix /tmp/myconda
```

Then ask the generated runtime for status:

```bash
dist/myconda status --prefix /tmp/myconda
```

The status output reports the binary name, prefix, configured channels,
configured package specs, installed package count, and conda executable path.

## Build An Embedded Artifact

Build an artifact that carries compressed package archives inside the binary:

```bash
pixi run -- cargo run -p pronto -- build --layout embedded --name myconda
```

The embedded artifact uses the `z` suffix by default, so the binary is staged as
`dist/mycondaz` on Unix and `dist/mycondaz.exe` on Windows.
