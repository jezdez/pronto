# pronto

Build ready-to-run conda bootstrap binaries.

`pronto` is a generic builder and runtime foundation for single-binary conda
distributions.

The intended artifact layouts are:

- `none`: `<name>` with embedded lock/metadata; packages are downloaded during bootstrap.
- `external`: `<name>` plus `<name>.bundle.tar.zst`.
- `embedded`: `<name>z`, the runtime plus compressed bundle embedded in one binary.

The local CLI mirrors the GitHub Actions workflow:

```bash
pronto lock
pronto inspect
pronto build --layout none --name myconda
pronto build --layout embedded --name myconda
pronto run --name myconda -- bootstrap --prefix /tmp/myconda-smoke
```

Every `pronto build` writes the staged binary plus artifact metadata: the
runtime lock, a tab-separated package list, an info JSON document, and SHA256
checksums. The GitHub Action uses the same build path and `embed-bundle: true`
for embedded builds.

Generic runtime behavior stays here; opinionated package sets and distribution
defaults belong in downstream distributions.

`pronto` is not an OS installer generator and does not target `.sh`, `.pkg`, or
`.msi` output. It produces bootstrap binaries that can be distributed directly
or wrapped by Homebrew, constructor, Docker, enterprise packaging systems, and
other release tooling.
