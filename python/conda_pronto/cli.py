"""CLI adapter used by the ``conda pronto`` plugin command."""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Sequence


def configure_parser(parser: argparse.ArgumentParser) -> None:
    """Configure the parser for ``conda pronto``."""
    parser.add_argument(
        "pronto_args",
        nargs=argparse.REMAINDER,
        metavar="ARGS",
        help="Arguments passed through to the pronto executable.",
    )


def execute(args: argparse.Namespace) -> int:
    """Run ``pronto`` and return its status code."""
    return run_pronto(args.pronto_args)


def main(argv: Sequence[str] | None = None) -> None:
    """Standalone debugging entry point for the plugin adapter."""
    raise SystemExit(run_pronto(sys.argv[1:] if argv is None else argv))


def run_pronto(argv: Sequence[str], *, executable: str | None = None) -> int:
    """Delegate to the canonical ``pronto`` executable."""
    pronto = executable or os.environ.get("CONDA_PRONTO_EXECUTABLE") or shutil.which("pronto")
    if pronto is None:
        print(
            "conda-pronto could not find the `pronto` executable on PATH.",
            file=sys.stderr,
        )
        return 127

    pronto_args = list(argv)
    if pronto_args[:1] == ["--"]:
        pronto_args = pronto_args[1:]
    if not pronto_args:
        pronto_args = ["--help"]

    try:
        return subprocess.run([pronto, *pronto_args]).returncode
    except FileNotFoundError:
        print(f"conda-pronto could not execute {pronto!r}.", file=sys.stderr)
        return 127


if __name__ == "__main__":
    main()
