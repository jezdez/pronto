# pronto

Build ready-to-run conda bootstrap binaries.

`pronto` is a generic builder and runtime template for single-binary conda
distributions. It does not ship a first-party distribution runtime. Downstream
projects choose the binary name, package set, channels, documentation URL, and
release channel.

Use these docs by goal:

- Start with the tutorial if you want a guided first build.
- Use how-to guides when you already know what you need to do.
- Use reference pages for exact command, action, configuration, and artifact
  details.
- Read explanation pages for the design model behind Pronto.

```{toctree}
:caption: Tutorials
:maxdepth: 1

tutorials/first-runtime
```

```{toctree}
:caption: How-To Guides
:maxdepth: 1

how-to/build-locally
how-to/build-in-github-actions
how-to/build-offline-artifacts
```

```{toctree}
:caption: Reference
:maxdepth: 1

reference/cli
reference/github-action
reference/configuration
reference/artifacts
```

```{toctree}
:caption: Explanation
:maxdepth: 1

explanation/concepts
explanation/runtime-template
```

```{toctree}
:caption: Project
:maxdepth: 1

roadmap
```
