# Roadmap

`pronto` is focused on the generic build system and runtime template for
single-binary conda distributions.

The builder CLI now covers the core local workflow:

- `pronto lock`: derive the runtime lock from the `runtime` Pixi environment
- `pronto inspect`: summarize the package set for a target platform
- `pronto bundle`: download package archives into a compressed bundle
- `pronto build`: stage `none`, `external`, or `embedded` artifacts
- `pronto run`: build and execute a local artifact for smoke testing

Every staged build writes the binary plus artifact metadata: the runtime lock,
a package list, an info JSON file, and SHA256 checksums.

Generic runtime behavior now lives in `pronto`; opinionated package sets and
distribution defaults belong in downstream projects.

The repository should stay focused on producing bootstrap binaries. Distribution
wrappers such as Homebrew formulae, constructor-based installers, Docker images,
or enterprise package manager recipes should live outside the core builder.
