#!/usr/bin/env python3
"""Verify the C# RuntimeIdentifiers set matches the build-ffi CI matrix.

``packages/csharp/LiterLlm/LiterLlm.csproj`` declares which RIDs the NuGet
package supports (``<RuntimeIdentifiers>``); ``.github/workflows/publish.yaml``'s
``build-ffi`` job matrix is what actually produces a native library per RID.
These two lists have drifted apart twice already (macos-x86_64, then
win-arm64: a RID declared in the csproj with no matching matrix leg, so a
consumer on that RID resolves no native asset and fails at load). Both sets
are extracted from their source files here rather than hardcoded, so this
check cannot itself go stale — a hardcoded third list would just be another
copy of the same data.

Usage:
    python3 scripts/ci/check_csharp_rid_drift.py    # exit 1 on drift
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
CSPROJ = ROOT / "packages" / "csharp" / "LiterLlm" / "LiterLlm.csproj"
PUBLISH_WORKFLOW = ROOT / ".github" / "workflows" / "publish.yaml"


def declared_rids() -> set[str]:
    """RIDs declared in LiterLlm.csproj's <RuntimeIdentifiers>.

    Matched with a regex rather than an XML parser: the element is a single
    semicolon-separated line, and a full parse would pull in an XML stack that
    ruff flags (S314) as unsafe on untrusted input for no gain here.
    """
    text = CSPROJ.read_text(encoding="utf-8")
    match = re.search(r"<RuntimeIdentifiers>([^<]+)</RuntimeIdentifiers>", text)
    if not match:
        raise SystemExit(f"no <RuntimeIdentifiers> element found in {CSPROJ}")
    return {rid.strip() for rid in match.group(1).split(";") if rid.strip()}


def built_rids() -> set[str]:
    """RIDs produced by the build-ffi job's matrix in publish.yaml."""
    text = PUBLISH_WORKFLOW.read_text(encoding="utf-8")
    # Isolate the build-ffi job body: from its header to the next top-level
    # (2-space-indented) job key, so a `rid:` field added to an unrelated
    # job elsewhere in this large workflow can never be picked up here.
    match = re.search(r"(?m)^  build-ffi:\n(.*?)(?=^  [A-Za-z][\w-]*:\n)", text, re.DOTALL)
    if not match:
        raise SystemExit(f"could not locate build-ffi job in {PUBLISH_WORKFLOW}")
    body = match.group(1)
    rids = re.findall(r"(?m)^\s*rid:\s*(\S+)\s*$", body)
    if not rids:
        raise SystemExit(f"no matrix `rid:` entries found in build-ffi job of {PUBLISH_WORKFLOW}")
    return set(rids)


def main() -> int:
    """Compare the two RID sets and report any drift.

    The printed lines are this check's machine-readable result, not incidental
    logging, so `print` is the correct surface here (ruff T201 suppressed per
    call for that reason).
    """
    declared = declared_rids()
    built = built_rids()

    declared_not_built = declared - built
    built_not_declared = built - declared

    if not declared_not_built and not built_not_declared:
        print(f"OK: {len(declared)} RIDs declared and built agree: {sorted(declared)}")  # noqa: T201
        return 0

    print(  # noqa: T201
        "RID drift between LiterLlm.csproj and publish.yaml's build-ffi matrix:",
        file=sys.stderr,
    )
    if declared_not_built:
        print(  # noqa: T201
            f"  declared in RuntimeIdentifiers but never built: {sorted(declared_not_built)}",
            file=sys.stderr,
        )
    if built_not_declared:
        print(  # noqa: T201
            f"  built by CI but not declared in RuntimeIdentifiers: {sorted(built_not_declared)}",
            file=sys.stderr,
        )
    return 1


if __name__ == "__main__":
    sys.exit(main())
