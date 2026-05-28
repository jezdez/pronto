# Build Offline Artifacts

Offline artifacts let the generated runtime install from package archives that
were downloaded during the build.

## Choose A Layout

Use `external` when you want the runtime and compressed bundle as separate
files:

```bash
pixi run -- cargo run -p pronto -- build --layout external --name myconda
```

Use `embedded` when you want one larger binary:

```bash
pixi run -- cargo run -p pronto -- build --layout embedded --name myconda
```

## Bootstrap From An External Bundle

For an `external` build, distribute these files together:

- `myconda`
- `myconda.bundle.tar.zst`
- `myconda.runtime.lock`
- `myconda.info.json`
- `myconda.packages.txt`
- `myconda.sha256`

Point the runtime at an extracted bundle directory:

```bash
myconda bootstrap --prefix /opt/myconda --bundle /path/to/packages --offline
```

Pronto also embeds a distribution-specific bundle environment variable in the
runtime. For a distribution named `myconda`, that variable is
`MYCONDA_BUNDLE`.

## Bootstrap From An Embedded Bundle

An embedded artifact carries the bundle inside the binary:

```bash
mycondaz bootstrap --prefix /opt/myconda --offline
```

The runtime extracts the compressed package archives to a temporary directory
during bootstrap.
