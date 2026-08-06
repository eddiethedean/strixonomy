"""Reserved PyPI package for the future Strixonomy Python SDK (v0.31).

This distribution does not expose ontology editing, querying, validation, diff,
or reasoning APIs. Use the Strixonomy CLI, LSP, or Rust crates for shipped
capabilities. See https://strixonomy.readthedocs.io/en/latest/guides/python-package/
"""

from __future__ import annotations

from typing import TypedDict

__version__ = "0.28.1"
__all__ = ["__version__", "status"]


class PackageStatus(TypedDict):
    package: str
    version: str
    reservation: bool
    sdk_planned_release: str
    documentation: str
    repository: str


def status() -> PackageStatus:
    """Return honest metadata about this reservation distribution."""
    return {
        "package": "strixonomy",
        "version": __version__,
        "reservation": True,
        "sdk_planned_release": "1.1",
        "documentation": "https://strixonomy.readthedocs.io/en/latest/guides/python-package/",
        "repository": "https://github.com/eddiethedean/strixonomy",
    }
