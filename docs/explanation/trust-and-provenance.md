# Trust And Provenance

conda-ship has a narrow trust model. It verifies the inputs it consumes and the
package archives it installs, but downstream distributions still own signing and
release policy for their final artifacts.

## Build Tool Trust

The GitHub Action downloads conda-ship release assets:

- `cs-<target>`
- `cs-template-<target>`
- `SHA256SUMS`

It verifies GitHub artifact attestations for those files and checks the
published SHA256 sums before running `cs`.

This protects the builder path from accidentally executing an unverified
downloaded binary in CI.

## Source Lock Trust

The source lockfile is committed project input. conda-ship assumes the
downstream project reviewed and committed that lockfile intentionally.

conda-ship does not solve loose matchspecs in the GitHub Action. That avoids a
release build changing package records because a channel changed between
workflow runs.

## Package Archive Trust

The runtime lock contains concrete package records. For bundle builds,
conda-ship requires SHA256 metadata so downloaded package archives can be
verified.

During bootstrap:

- online installs use the stamped runtime lock
- external bundle installs match local package archives to the lock
- embedded bundle installs verify the embedded bundle before extraction

The runtime rejects package archive mismatches instead of silently installing
unexpected files.

## Runtime Artifact Trust

Every staged build writes checksums and metadata:

- `.sha256`
- `.info.json`
- `.runtime.lock`
- `.packages.txt`

These files describe and verify what conda-ship produced. They are not a
replacement for signing.

## Downstream Signing

Sign after conda-ship has staged the final files. Good places for downstream
signing include:

- GitHub Release artifact attestations
- Sigstore signatures
- in-toto provenance for release workflows
- platform-specific installer signing
- package-manager-specific signatures or checksums

Signing belongs downstream because one runtime can be distributed through
several channels, and each channel has different trust requirements.

## What conda-ship Does Not Promise

conda-ship does not:

- decide which channels are trusted for a downstream distribution
- sign downstream runtime artifacts
- make a wrapper installer trustworthy by itself
- replace review of committed source lockfiles
- hide the need for package-manager or platform signing

It provides reproducible build output, narrow runtime verification, and metadata
that downstream release systems can sign.

