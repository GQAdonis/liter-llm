#!/usr/bin/env python3
"""Check crates/liter-llm/Cargo.toml's hardcoded fields stay in sync with the
workspace tables they're supposed to mirror.

crates/liter-llm's ``[package]`` fields and several of its ``[dependencies]``/
``[dev-dependencies]`` entries are hardcoded (``version = "x"``) instead of
``*.workspace = true``, because Cargo resolves workspace inheritance against
the *enclosing* workspace for any path dependency nested inside a workspace's
directory tree — breaking this crate when it's vendored as a nested path
dependency (e.g. universal-agent-runtime's vendor/git/liter-llm). See the
``~keep``/``NOTE`` comments on those fields in crates/liter-llm/Cargo.toml.

Because those values are hardcoded, nothing re-syncs them automatically when
the root Cargo.toml's [workspace.package]/[workspace.dependencies] tables
change (e.g. a models.dev catalog sync bumping a dependency version) — they
silently drift until a build breaks. This compares every hardcoded name found
in crates/liter-llm/Cargo.toml against the workspace tables directly, so it
needs no maintained allowlist as more fields/deps get hardcoded over time.

Usage:
    python3 scripts/check_vendored_deps.py           # report drift, exit 1 if any
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ROOT_MANIFEST = ROOT / "Cargo.toml"
CRATE_MANIFEST = ROOT / "crates" / "liter-llm" / "Cargo.toml"

# [package] fields that the hardcoding NOTE in crates/liter-llm/Cargo.toml claims
# mirror [workspace.package] (identity/versioning fields). Free-text fields like
# description/keywords/categories/readme are legitimately authored per-crate for
# the published crates.io listing and are intentionally excluded here.
PACKAGE_FIELDS_TO_CHECK = {"version", "edition", "license", "repository", "homepage"}

NAME_VALUE_RE = re.compile(r'(?m)^([A-Za-z0-9_.\-]+)\s*=\s*"([^"]+)"\s*$')
NAME_INLINE_TABLE_VERSION_RE = re.compile(
    r'(?m)^([A-Za-z0-9_\-]+)\s*=\s*\{[^}\n]*\bversion\s*=\s*"([^"]+)"'
)


def extract_table(text: str, header: str) -> str:
    """Return the raw text of a top-level TOML table, up to the next
    top-level ``[...]`` header or end of file."""
    pattern = re.compile(rf"(?ms)^\[{re.escape(header)}\]\s*\n(.*?)(?=^\[|\Z)")
    match = pattern.search(text)
    return match.group(1) if match else ""


def collect_values(table_text: str) -> dict[str, str]:
    """Collect every ``name = "value"`` and ``name = { ..., version = "value", ... }``
    entry in a table, keyed by name. Plain string fields (used in [package]) and
    dependency version pins (used in [dependencies]/[dev-dependencies]) share the
    same ``name = "..."`` shape, so one pass covers both kinds of table."""
    values: dict[str, str] = {}
    for name, value in NAME_VALUE_RE.findall(table_text):
        values[name] = value
    for name, value in NAME_INLINE_TABLE_VERSION_RE.findall(table_text):
        values[name] = value
    return values


def diff_against_workspace(ws_values: dict[str, str], crate_values: dict[str, str], label: str) -> list[str]:
    drift = []
    for name, crate_value in crate_values.items():
        ws_value = ws_values.get(name)
        if ws_value is not None and ws_value != crate_value:
            drift.append(f"{label}.{name}: crate={crate_value!r} workspace={ws_value!r}")
    return drift


def main() -> int:
    root_text = ROOT_MANIFEST.read_text(encoding="utf-8")
    crate_text = CRATE_MANIFEST.read_text(encoding="utf-8")

    ws_package = collect_values(extract_table(root_text, "workspace.package"))
    ws_deps = collect_values(extract_table(root_text, "workspace.dependencies"))

    crate_package = {
        name: value
        for name, value in collect_values(extract_table(crate_text, "package")).items()
        if name in PACKAGE_FIELDS_TO_CHECK
    }
    crate_deps = collect_values(extract_table(crate_text, "dependencies"))
    crate_dev_deps = collect_values(extract_table(crate_text, "dev-dependencies"))

    drift = [
        *diff_against_workspace(ws_package, crate_package, "[package]"),
        *diff_against_workspace(ws_deps, crate_deps, "[dependencies]"),
        *diff_against_workspace(ws_deps, crate_dev_deps, "[dev-dependencies]"),
    ]

    if drift:
        print(
            "crates/liter-llm/Cargo.toml has hardcoded values that drifted from "
            "the workspace tables (see the NOTE/~keep comments on those fields):"
        )
        for line in sorted(drift):
            print(f"  {line}")
        print(
            "\nUpdate the hardcoded values in crates/liter-llm/Cargo.toml to match, "
            "then re-run `cargo check --workspace` to refresh Cargo.lock."
        )
        return 1

    print("crates/liter-llm/Cargo.toml hardcoded fields are in sync with the workspace.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
