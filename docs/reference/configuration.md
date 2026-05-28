# Configuration Reference

Pronto reads runtime package intent from `pixi.toml` and concrete package
records from `pixi.lock`.

## Pixi Runtime Environment

The `runtime` environment determines the conda packages available to the
generated bootstrap runtime:

```toml
[feature.runtime.dependencies]
python = ">=3.12"
conda = ">=25.1"
conda-rattler-solver = "*"
conda-spawn = ">=0.1.0"

[environments]
runtime = { features = ["runtime"], no-default-feature = true }
```

## `[tool.pronto]`

`[tool.pronto]` records the runtime-facing package specs, channels, and
exclusions:

```toml
[tool.pronto]
channels = ["conda-forge"]
packages = [
  "python >=3.12",
  "conda >=25.1",
  "conda-rattler-solver",
  "conda-spawn >=0.1.0",
]
exclude = ["conda-libmamba-solver"]
```

`packages`
: Specs shown in runtime metadata and used when building without the embedded
  lock.

`channels`
: Channels written into runtime metadata.

`exclude`
: Package names removed from the derived runtime lock, including dependencies
  used only by excluded packages.

## Build-Time Runtime Metadata

`pronto build --name NAME` embeds these runtime values:

- command name: `NAME` for `none` and `external`, `NAME` plus `z` for
  `embedded`
- display name: `NAME`
- default prefix: `~/.NAME`
- metadata file: `.NAME.json`
- bundle environment variable: uppercased `NAME` plus `_BUNDLE`
- offline environment variable: uppercased `NAME` plus `_OFFLINE`

Non-alphanumeric characters in environment variable names become underscores.
