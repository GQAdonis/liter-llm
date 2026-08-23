#!/usr/bin/env python3
"""Verify the three C# RID lists agree: declared, built, and packed.

A RID has to survive all three stages for a consumer to load the binding:

1. ``packages/csharp/LiterLlm/LiterLlm.csproj`` declares the supported set
   (``<RuntimeIdentifiers>``), and the meta package's runtime.json names an
   ``XbergIo.LiterLlm.runtime.<rid>`` package for each.
2. ``.github/workflows/publish.yaml``'s ``build-ffi`` matrix produces the
   native library.
3. ``build-csharp-package``'s ``CSHARP_RIDS`` stages that native and packs it
   into the per-RID runtime package.

1 and 2 drifted apart twice (macos-x86_64, then win-arm64: a RID declared with
no matching matrix leg, so that RID resolved no native asset). Stage 3 is worse
when it drifts and was missed entirely for a while — the runtime packages were
never packed at all, so runtime.json pointed at packages that did not exist on
nuget.org for *every* RID and the binding could not load anywhere.

All three sets are extracted from their source files rather than hardcoded, so
this check cannot itself go stale — a hardcoded fourth list would just be
another copy of the same data.

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


def packaged_rids() -> set[str]:
    """RIDs the build-csharp-package job stages natives for and packs.

    A RID can be declared and built yet never packed into an
    ``XbergIo.LiterLlm.runtime.<rid>`` package, which is exactly the failure this
    check was extended for: the meta package's runtime.json names a runtime
    package per RID, and any RID missing from the pack loop resolves to a package
    that does not exist on nuget.org.
    """
    text = PUBLISH_WORKFLOW.read_text(encoding="utf-8")
    match = re.search(r"(?m)^  build-csharp-package:\n(.*?)(?=^  [A-Za-z][\w-]*:\n)", text, re.DOTALL)
    if not match:
        raise SystemExit(f"could not locate build-csharp-package job in {PUBLISH_WORKFLOW}")
    env_match = re.search(r"(?m)^\s*CSHARP_RIDS:\s*(.+?)\s*$", match.group(1))
    if not env_match:
        raise SystemExit(f"no job-level CSHARP_RIDS declared in build-csharp-package job of {PUBLISH_WORKFLOW}")
    return {rid for rid in env_match.group(1).split() if rid}


def main() -> int:
    """Compare the three RID sets and report any drift.

    The printed lines are this check's machine-readable result, not incidental
    logging, so `print` is the correct surface here (ruff T201 suppressed per
    call for that reason).
    """
    sets = {
        "declared in LiterLlm.csproj <RuntimeIdentifiers>": declared_rids(),
        "built by the build-ffi matrix": built_rids(),
        "staged and packed via CSHARP_RIDS": packaged_rids(),
    }

    union = set().union(*sets.values())
    drift = {label: sorted(union - rids) for label, rids in sets.items() if union - rids}

    if not drift:
        print(f"OK: {len(union)} RIDs agree across all three sources: {sorted(union)}")  # noqa: T201
        return 0

    print("C# RID drift detected:", file=sys.stderr)  # noqa: T201
    for label, missing in drift.items():
        print(f"  missing from {label}: {missing}", file=sys.stderr)  # noqa: T201
    return 1


if __name__ == "__main__":
    sys.exit(main())
