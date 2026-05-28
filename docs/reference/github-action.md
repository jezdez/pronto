# GitHub Action Reference

The repository root provides a composite GitHub Action.

```yaml
- uses: jezdez/pronto@main
  id: pronto
  with:
    name: myconda
```

## Inputs

`name`
: Required distribution binary name.

`packages`
: Optional comma-separated conda package specs. When omitted, Pronto uses the
  package specs in its runtime configuration.

`channels`
: Optional comma-separated conda channels. When omitted, Pronto uses the
  configured channels.

`exclude`
: Optional comma-separated package names to remove from the generated runtime
  lock, including exclusive dependencies.

`ref`
: Git ref of Pronto to build from. Defaults to `main`.

`embed-bundle`
: Set to `"true"` to embed package archives into the runtime binary.

`docs-url`
: Documentation URL embedded in the generated runtime help output.

## Outputs

`binary-path`
: Absolute path to the generated runtime binary.

`asset-name`
: Platform-qualified asset filename.

`info-path`
: Absolute path to the artifact info JSON.

`lock-path`
: Absolute path to the staged runtime lock.

`package-list-path`
: Absolute path to the staged package list.

`checksums-path`
: Absolute path to the SHA256 checksum file.
