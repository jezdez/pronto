# Artifact Reference

Every `pronto build` writes a runtime binary plus metadata files.

## Layouts

| Layout | Binary name | Bundle file | Network during bootstrap |
| --- | --- | --- | --- |
| `none` | `NAME` | none | yes |
| `external` | `NAME` | `NAME.bundle.tar.zst` | optional |
| `embedded` | `NAMEz` | embedded in binary | no, when used with `--offline` |

On Windows, binary filenames also include `.exe`.

## Metadata Files

For a build named `myconda`, Pronto stages:

- `myconda` or `myconda.exe`
- `myconda.runtime.lock`
- `myconda.packages.txt`
- `myconda.info.json`
- `myconda.sha256`

When `--target-label` is used, the label is inserted into the stem, for example
`myconda-linux-64.info.json`.

## Info JSON

The info JSON contains:

- schema version
- artifact name
- layout
- conda platform
- binary filename
- optional bundle filename
- lock filename
- package list filename
- package count
- SHA256 checksums

## Package List

The package list is tab-separated and contains:

- package name
- version
- build string
- package URL
- SHA256, when available from the lockfile
