# Verify Release Artifacts

Use this guide when you need to check conda-ship-built artifacts before
publishing or wrapping them.

Verification has two layers:

- verify the conda-ship tools used by the build
- verify the runtime artifacts produced by the build

## Verify conda-ship Release Tools In GitHub Actions

The composite action downloads `cs`, `cs-template`, and `SHA256SUMS`
from a tagged conda-ship release. It verifies GitHub artifact attestations and
then checks SHA256 sums before running `cs`.

Use a tag:

```yaml
- uses: jezdez/conda-ship@v0.1.0
```

Do not use a branch ref for release builds. Branch refs do not have matching
release assets.

Self-hosted runners must provide the GitHub CLI because the action calls
`gh attestation verify`.

## Verify Staged Checksums

Every `cs build` writes a `.sha256` file next to the runtime and metadata:

```bash
shasum -a 256 --check dist/demo.sha256
```

On Linux, `sha256sum` works too:

```bash
sha256sum --check dist/demo.sha256
```

The checksum file covers the staged runtime, runtime lock, package list, info
JSON, and external bundle when present.

## Inspect Artifact Metadata

Open the `.info.json` file:

```bash
python -m json.tool dist/demo.info.json
```

Check:

- `name`
- `layout`
- `platform`
- `binary`
- `bundle`
- `package_count`
- `checksums`

This file is intended for release tooling and package-manager wrappers. It
describes what conda-ship wrote, not what an external installer later did.

## Inspect The Runtime Lock

The staged `.runtime.lock` is the lock the runtime will use during bootstrap.
It should be reproducible from the committed source lockfile and
`[tool.conda-ship]`.

Use it to answer release questions such as:

- Which concrete conda packages are shipped?
- Which channels are recorded?
- Which platforms are present?

Do not edit it by hand. Change the source manifest or source lockfile instead,
then rebuild.

## Verify Bundle Contents

For external bundles, extract into a temporary directory and check that it
contains only top-level package archives:

```bash
mkdir -p /tmp/demo-bundle
tar --zstd -xf dist/demo.bundle.tar.zst -C /tmp/demo-bundle
find /tmp/demo-bundle -maxdepth 2 -type f
```

The runtime verifies package archive hashes against the runtime lock before
installing. Embedded bundles are verified by the runtime before extraction.

## Add Downstream Signing

conda-ship does not sign downstream runtime artifacts. Sign after `cs build`,
when the final files are staged and checksums are written.

Good downstream signing points are:

- GitHub Release artifact attestations
- Sigstore signing for uploaded artifacts
- in-toto provenance around the packaging workflow
- platform-specific signing for installer wrappers

Keep signing outside the generic runtime. A runtime built by conda-ship may be
wrapped by several downstream channels, and each channel owns its own trust
policy.

