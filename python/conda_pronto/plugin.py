"""Conda plugin hooks for conda-pronto.

This module is imported on every conda invocation through the conda plugin
entry point. Keep imports light and defer the adapter implementation until the
subcommand is actually configured.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from conda.plugins import hookimpl

if TYPE_CHECKING:
    from collections.abc import Iterable

    from conda.plugins.types import CondaSubcommand


@hookimpl
def conda_subcommands() -> Iterable[CondaSubcommand]:
    from conda.plugins.types import CondaSubcommand

    from .cli import configure_parser, execute

    yield CondaSubcommand(
        name="pronto",
        summary="Build ready-to-run conda bootstrap binaries.",
        action=execute,
        configure_parser=configure_parser,
    )
