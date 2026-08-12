#!/usr/bin/env python3
"""Assert that every Taskfile poly skips is skipped for the reason we expect.

poly cannot format Taskfile YAML that contains Go template syntax (``{{ }}``)
and skips those files. That is correct behavior — the files genuinely are not
parseable as plain YAML — but it means most of our task automation is linted by
nothing. The danger is not the skip, it is that a *new* file can silently join
the skipped set and look identical to a file that was checked and found clean.

poly reports only a count, not the names, so this check reconstructs the
expected set from the property that causes the skip:

    expected skipped = files in the scan set containing ``{{``

and asserts poly's reported counts match. A file skipped for any *other* reason
— a parse failure, a guard change, a new engine restriction — moves the actual
count away from the expected one and fails here with the delta named.

Deliberately not a hardcoded ``17``: that would go stale the first time someone
adds or removes a task file, and a check that has to be edited whenever the tree
changes gets edited to whatever makes it pass. This version self-updates.

Retire this in favour of a poly flag (``--deny-skips`` / ``--max-skips=N``) if
one ships; it was requested upstream. This is a workaround for a missing
strict mode, not a thing worth keeping on its own merits.

Usage:
    python3 scripts/ci/check_task_lint_coverage.py    # exit 1 on unexpected skips
"""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
TEMPLATE_MARKER = "{{"


def scan_set() -> list[Path]:
    """The files handed to poly: the root Taskfile plus every included task file."""
    paths = sorted(ROOT.joinpath(".task").rglob("*.yml"))
    root_taskfile = ROOT / "Taskfile.yml"
    if root_taskfile.exists():
        paths.append(root_taskfile)
    if not paths:
        raise SystemExit(f"no task files found under {ROOT}; the scan set cannot be empty")
    return paths


def expected_skipped(paths: list[Path]) -> list[Path]:
    """Files poly is expected to skip, i.e. those carrying Go template syntax."""
    return [p for p in paths if TEMPLATE_MARKER in p.read_text(encoding="utf-8")]


def poly_counts(paths: list[Path]) -> tuple[int, int]:
    """Run poly and return its (checked, skipped) counts.

    ``--no-cache`` because a cache hit changes what the summary reports, which is
    the exact trap that made an earlier version of this check give a false pass.
    """
    poly = shutil.which("poly")
    if poly is None:
        raise SystemExit("poly is not on PATH; run `task setup` first")

    argv = [poly, "fmt", "--check", "--no-cache", *[str(p) for p in paths]]
    result = subprocess.run(argv, capture_output=True, text=True, check=False)
    output = result.stdout + result.stderr

    match = re.search(r"(\d+) file\(s\) checked(?:, (\d+) skipped)?", output)
    if not match:
        raise SystemExit(
            "could not parse poly's summary line; its output format changed.\n"
            "This check must fail loudly rather than assume zero skips.\n"
            f"poly said:\n{output}"
        )
    return int(match.group(1)), int(match.group(2) or 0)


def main() -> int:
    """Compare poly's skip count against the templated-file count.

    The printed lines are this check's machine-readable result, not incidental
    logging, so `print` is the correct surface here (ruff T201 suppressed per
    call for that reason).
    """
    paths = scan_set()
    expected = expected_skipped(paths)
    checked, skipped = poly_counts(paths)

    if skipped == len(expected) and checked == len(paths) - len(expected):
        print(  # noqa: T201
            f"OK: {checked} task file(s) linted, {skipped} skipped, "
            f"and all {skipped} skips are explained by Go template syntax."
        )
        return 0

    print("Task lint coverage changed unexpectedly:", file=sys.stderr)  # noqa: T201
    print(  # noqa: T201
        f"  poly reports   {checked} checked, {skipped} skipped (of {len(paths)} file(s))",
        file=sys.stderr,
    )
    print(  # noqa: T201
        f"  expected       {len(paths) - len(expected)} checked, {len(expected)} skipped",
        file=sys.stderr,
    )
    print(  # noqa: T201
        "  Files carrying Go template syntax, which are the ones poly is expected to skip:",
        file=sys.stderr,
    )
    for path in expected:
        print(f"    {path.relative_to(ROOT)}", file=sys.stderr)  # noqa: T201
    print(  # noqa: T201
        "\nIf poly skipped MORE than that, a file is being skipped for a reason other than\n"
        "templating and is silently unlinted — find it before assuming this is benign.",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
