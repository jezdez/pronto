from __future__ import annotations

import argparse
from types import SimpleNamespace
from typing import TYPE_CHECKING

import pytest
from conda_pronto import cli
from conda_pronto.cli import configure_parser, execute, run_pronto

if TYPE_CHECKING:
    from collections.abc import Sequence


def test_configure_parser_collects_pronto_args() -> None:
    parser = argparse.ArgumentParser(prog="conda pronto")
    configure_parser(parser)

    args = parser.parse_args(["build", "--layout", "none", "--name", "serpe"])

    assert args.pronto_args == ["build", "--layout", "none", "--name", "serpe"]


@pytest.mark.parametrize(
    ("argv", "expected"),
    [
        pytest.param(
            ["build", "--name", "serpe"],
            ["/tmp/pronto", "build", "--name", "serpe"],
            id="args",
        ),
        pytest.param(["--"], ["/tmp/pronto", "--help"], id="separator-defaults-to-help"),
        pytest.param([], ["/tmp/pronto", "--help"], id="empty-defaults-to-help"),
    ],
)
def test_run_pronto_delegates_to_executable(
    argv: Sequence[str],
    expected: list[str],
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    calls: list[list[str]] = []

    def fake_run(args: list[str]) -> SimpleNamespace:
        calls.append(args)
        return SimpleNamespace(returncode=17)

    monkeypatch.setattr(cli.subprocess, "run", fake_run)

    status = run_pronto(argv, executable="/tmp/pronto")

    assert status == 17
    assert calls == [expected]


def test_run_pronto_reports_missing_executable(
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    monkeypatch.setattr(cli.shutil, "which", lambda _name: None)

    status = run_pronto([])

    assert status == 127
    assert "could not find" in capsys.readouterr().err


def test_run_pronto_prefers_current_environment_binary(
    tmp_path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    bin_dir = tmp_path / "bin"
    bin_dir.mkdir()
    python = bin_dir / "python"
    pronto = bin_dir / ("pronto.exe" if cli.os.name == "nt" else "pronto")
    python.write_text("")
    pronto.write_text("")
    calls: list[list[str]] = []

    def fake_run(args: list[str]) -> SimpleNamespace:
        calls.append(args)
        return SimpleNamespace(returncode=0)

    monkeypatch.setattr(cli.sys, "executable", str(python))
    monkeypatch.setattr(cli.shutil, "which", lambda _name: "/tmp/path-pronto")
    monkeypatch.setattr(cli.subprocess, "run", fake_run)

    assert run_pronto(["inspect"]) == 0
    assert calls == [[str(pronto), "inspect"]]


def test_execute_returns_pronto_status(monkeypatch: pytest.MonkeyPatch) -> None:
    calls: list[list[str]] = []

    def fake_run_pronto(args: Sequence[str]) -> int:
        calls.append(list(args))
        return 3

    monkeypatch.setattr(cli, "run_pronto", fake_run_pronto)
    args = argparse.Namespace(pronto_args=["inspect"])

    assert execute(args) == 3
    assert calls == [["inspect"]]
