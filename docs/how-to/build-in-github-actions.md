# Build In GitHub Actions

Use the composite action when a downstream distribution repository wants Pronto
to build release artifacts in CI.

## Single-Platform Example

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: jezdez/pronto@main
        id: pronto
        with:
          name: myconda
          packages: "python >=3.12, conda >=25.1"
          channels: "conda-forge"

      - uses: actions/upload-artifact@v4
        with:
          name: ${{ steps.pronto.outputs.asset-name }}
          path: |
            ${{ steps.pronto.outputs.binary-path }}
            ${{ steps.pronto.outputs.info-path }}
            ${{ steps.pronto.outputs.lock-path }}
            ${{ steps.pronto.outputs.package-list-path }}
            ${{ steps.pronto.outputs.checksums-path }}
```

Pin `jezdez/pronto` to a tag or commit SHA for release builds.

## Embedded Bundle Example

Set `embed-bundle` when the runtime must bootstrap without network access:

```yaml
- uses: jezdez/pronto@main
  id: pronto
  with:
    name: myconda
    embed-bundle: "true"
```

The output binary uses the `z` suffix by default, for example `mycondaz` on Unix
or `mycondaz.exe` on Windows.

## Matrix Builds

Run the action across operating systems to produce platform-specific binaries:

```yaml
strategy:
  fail-fast: false
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]

runs-on: ${{ matrix.os }}

steps:
  - uses: jezdez/pronto@main
    id: pronto
    with:
      name: myconda
```

Each job emits an asset name qualified with the runner target triple.
