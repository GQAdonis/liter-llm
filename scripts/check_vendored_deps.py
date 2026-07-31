#!/usr/bin/env python3
"""Check crates/liter-llm/Cargo.toml's hardcoded fields stay in sync with the
workspace tables they're supposed to mirror.

crates/liter-llm's ``[package]`` fields, several of its ``[dependencies]``/
``[dev-dependencies]`` entries, and its ``[lints.clippy]``/``[lints.rust]``
tables are hardcoded (literal values) instead of ``*.workspace = true``,
because Cargo resolves workspace inheritance against the *enclosing* workspace
for any path dependency nested inside a workspace's directory tree — breaking
this crate when it's vendored as a nested path dependency (e.g.
universal-agent-runtime's vendor/git/liter-llm). See the ``~keep``/``NOTE``
comments on those fields in crates/liter-llm/Cargo.toml.

Because those values are hardcoded, nothing re-syncs them automatically when
the root Cargo.toml's [workspace.package]/[workspace.dependencies]/
[workspace.lints.*] tables change (e.g. a models.dev catalog sync bumping a
dependency version, or a new workspace-wide clippy lint) — they silently drift
until a build breaks or a lint gap goes unnoticed. This script covers three
distinct drift classes:

1. [package]/[dependencies]/[dev-dependencies] value mismatches: for every
   name hardcoded in crates/liter-llm/Cargo.toml that also appears in the
   workspace tables, the two values must be identical. No maintained allowlist
   needed — any hardcoded name is checked automatically.
2. [lints.clippy]/[lints.rust] omissions: unlike deps, lints should be a
   strict mirror, so this direction is reversed — every name present in the
   *workspace's* lint tables must also be present (with the same value) in the
   crate's copy. A workspace lint silently missing from the crate's copy (e.g.
   a newly-added `deny` lint) is exactly the kind of gap a "do both sides
   share a value where present" check would miss.
3. Dangling [features] references: crates/liter-llm/Cargo.toml's [features]
   table is NOT meant to mirror any workspace table (it's genuinely
   crate-owned) — but feature arrays reference other feature names and
   `dep:xxx` dependency names, and those references can go stale when a
   feature or dependency is renamed/removed elsewhere in the same file (e.g.
   upstream removing a `tracing` feature while another feature array still
   listed `"tracing"` as a bare feature reference). This flags any reference
   that doesn't resolve to a real feature or dependency.

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
# A lint entry is either `name = "level"` or `name = { ...whole inline table... }`.
# Unlike dependency tables, we compare the ENTIRE inline table text (not just one
# sub-field) since lint tables carry no "version"-like anchor field.
LINT_ENTRY_RE = re.compile(r'(?m)^([A-Za-z0-9_\-]+)\s*=\s*(".*"|\{[^}\n]*\})\s*$')
FEATURE_ARRAY_RE = re.compile(r'(?ms)^([A-Za-z0-9_\-]+)\s*=\s*\[(.*?)\]')
FEATURE_REF_RE = re.compile(r'"([^"]+)"')


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


def collect_lint_entries(table_text: str) -> dict[str, str]:
    """Collect every lint name -> raw value (string literal or whole inline
    table, as written) in a [lints.*] table."""
    return dict(LINT_ENTRY_RE.findall(table_text))


def diff_lints_missing_from_crate(
    ws_lints: dict[str, str], crate_lints: dict[str, str], label: str
) -> list[str]:
    """Lints should be a strict mirror of the workspace, so unlike dependency
    versions the check runs workspace -> crate: every workspace lint must be
    present in the crate's copy with an identical value. A lint newly added to
    the workspace but absent from the crate's hardcoded copy is exactly the
    silent gap that let print_stdout/print_stderr/dbg_macro slip through."""
    drift = []
    for name, ws_value in ws_lints.items():
        crate_value = crate_lints.get(name)
        if crate_value is None:
            drift.append(f"{label}.{name}: missing from crate copy (workspace={ws_value!r})")
        elif crate_value != ws_value:
            drift.append(f"{label}.{name}: crate={crate_value!r} workspace={ws_value!r}")
    return drift


def collect_feature_arrays(table_text: str) -> dict[str, list[str]]:
    """Collect every `name = [...]` feature array in a [features] table,
    mapping the feature name to its list of quoted string references."""
    arrays: dict[str, list[str]] = {}
    for name, body in FEATURE_ARRAY_RE.findall(table_text):
        arrays[name] = FEATURE_REF_RE.findall(body)
    return arrays


def find_dangling_feature_refs(feature_arrays: dict[str, list[str]], dep_names: set[str]) -> list[str]:
    """A reference inside a feature array is either another feature name, or a
    `dep:xxx` activating a real dependency. Anything else is a dangling
    reference left behind by a rename/removal elsewhere in the file (e.g. a
    feature removed from [features] while another array still lists it as a
    bare string)."""
    feature_names = set(feature_arrays)
    drift = []
    for owner, refs in feature_arrays.items():
        for ref in refs:
            if ref in feature_names:
                continue
            if ref.startswith("dep:"):
                if ref[len("dep:") :] in dep_names:
                    continue
            elif "/" in ref:
                # Cross-crate feature activation (e.g. "liter-llm-proxy/otel"); not
                # resolvable from this file alone, so not checked.
                continue
            drift.append(f"[features].{owner}: dangling reference {ref!r} (no matching feature or dependency)")
    return drift


def main() -> int:
    root_text = ROOT_MANIFEST.read_text(encoding="utf-8")
    crate_text = CRATE_MANIFEST.read_text(encoding="utf-8")

    ws_package = collect_values(extract_table(root_text, "workspace.package"))
    ws_deps = collect_values(extract_table(root_text, "workspace.dependencies"))
    ws_clippy_lints = collect_lint_entries(extract_table(root_text, "workspace.lints.clippy"))
    ws_rust_lints = collect_lint_entries(extract_table(root_text, "workspace.lints.rust"))

    crate_package = {
        name: value
        for name, value in collect_values(extract_table(crate_text, "package")).items()
        if name in PACKAGE_FIELDS_TO_CHECK
    }
    crate_deps_table = extract_table(crate_text, "dependencies")
    crate_dev_deps_table = extract_table(crate_text, "dev-dependencies")
    crate_deps = collect_values(crate_deps_table)
    crate_dev_deps = collect_values(crate_dev_deps_table)
    crate_clippy_lints = collect_lint_entries(extract_table(crate_text, "lints.clippy"))
    crate_rust_lints = collect_lint_entries(extract_table(crate_text, "lints.rust"))
    crate_feature_arrays = collect_feature_arrays(extract_table(crate_text, "features"))

    # Dependency names referenced by `dep:xxx` in [features] can come from either
    # [dependencies] or target-conditional dependency tables (e.g. rustls/reqwest
    # are declared under [target.'cfg(...)'.dependencies], not [dependencies]).
    all_dep_names = set(crate_deps) | set(crate_dev_deps)
    for match in re.finditer(r"(?ms)^\[target\.[^\]]+\.dependencies\]\s*\n(.*?)(?=^\[|\Z)", crate_text):
        all_dep_names |= set(collect_values(match.group(1)))

    drift = [
        *diff_against_workspace(ws_package, crate_package, "[package]"),
        *diff_against_workspace(ws_deps, crate_deps, "[dependencies]"),
        *diff_against_workspace(ws_deps, crate_dev_deps, "[dev-dependencies]"),
        *diff_lints_missing_from_crate(ws_clippy_lints, crate_clippy_lints, "[lints.clippy]"),
        *diff_lints_missing_from_crate(ws_rust_lints, crate_rust_lints, "[lints.rust]"),
        *find_dangling_feature_refs(crate_feature_arrays, all_dep_names),
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
