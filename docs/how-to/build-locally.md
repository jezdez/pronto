# Build Locally

Use local builds while iterating on runtime package sets, channel choices, or
Pronto runtime code.

## Refresh The Artifact Lock

Run this after changing `pixi.toml`, `pixi.lock`, or `[tool.pronto]`:

```bash
pixi run prepare
```

CI checks the generated files with:

```bash
pixi run check-lock
```

## Build A Named Distribution Binary

`--name` is required. Pronto does not provide a default distribution name.

```bash
pixi run -- cargo run -p pronto -- build --layout none --name myconda
```

Use `--out-dir` to stage somewhere other than `dist/`:

```bash
pixi run -- cargo run -p pronto -- build \
  --layout none \
  --name myconda \
  --out-dir /tmp/pronto-artifacts
```

## Run A Smoke Test

Use `pronto run` to build and immediately execute the staged runtime:

```bash
pixi run -- cargo run -p pronto -- run \
  --name myconda \
  -- bootstrap --prefix /tmp/myconda-smoke
```

Everything after `--` is passed to the generated runtime.

## Cross-Compile With A Rust Target

Pass both the Rust target triple and an artifact label:

```bash
pixi run -- cargo run -p pronto -- build \
  --name myconda \
  --target x86_64-unknown-linux-gnu \
  --target-label x86_64-unknown-linux-gnu
```

The target label is appended to staged artifact names and metadata files.
