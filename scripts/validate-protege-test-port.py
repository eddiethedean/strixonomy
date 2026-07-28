#!/usr/bin/env python3
"""Validate parity/protege-test-port.yaml (PORT_W* rows need tests or gap).

Usage:
  python3 scripts/validate-protege-test-port.py
"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from parity_common import ROOT  # noqa: E402
import yaml  # noqa: E402

INVENTORY = ROOT / "parity" / "protege-test-port.yaml"
PORT_TAGS = ("PORT_W1", "PORT_W2", "PORT_W3", "PORT_W4")


def main() -> int:
    if not INVENTORY.is_file():
        print(f"missing inventory: {INVENTORY}", file=sys.stderr)
        return 1

    data = yaml.safe_load(INVENTORY.read_text(encoding="utf-8"))
    entries = data.get("entries") or []
    errors: list[str] = []
    counts = {t: 0 for t in PORT_TAGS}
    for i, entry in enumerate(entries):
        if not isinstance(entry, dict):
            errors.append(f"entries[{i}] must be a mapping")
            continue
        tag = entry.get("tag")
        name = entry.get("class", f"entries[{i}]")
        if tag in PORT_TAGS:
            counts[tag] += 1
            tests = entry.get("strixonomy_tests") or []
            gap = entry.get("gap")
            if not tests and not gap:
                errors.append(f"{name}: {tag} requires strixonomy_tests or gap")
            for t in tests:
                path = ROOT / t
                if not path.exists():
                    errors.append(f"{name}: missing strixonomy_tests path {t}")
        if tag not in (*PORT_TAGS, "SKIP", "COVERED"):
            errors.append(f"{name}: invalid tag {tag!r}")

    if errors:
        print(f"protege-test-port: {len(errors)} error(s)", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1

    summary = ", ".join(f"{counts[t]} {t}" for t in PORT_TAGS)
    print(f"protege-test-port: ok ({len(entries)} entries, {summary})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
