"""MkDocs hook: fix git-revision-date on Git builds where lone ``--diff-filter=r`` matches nothing.

``mkdocs-git-revision-date-localized`` passes ``diff_filter="r"`` (exclude-only).
On Apple Git / recent Git, a filter that is **only** lowercase letters returns no
commits, so the plugin falls back to ``time.time()`` and logs
``has no git logs, using current timestamp`` for ``docs/``.

Stripping that filter restores real revision stamps and removes the noise so
``mkdocs build --strict`` stays quiet.
"""

from __future__ import annotations

from typing import Any


def on_startup(**kwargs: Any) -> None:
    try:
        from git.cmd import Git
    except ImportError:
        return

    if getattr(Git, "_strixonomy_diff_filter_patch", False):
        return

    original = Git._call_process

    def _call_process(self: Any, method: str, *args: Any, **kwargs: Any) -> Any:
        if method == "log" and kwargs.get("diff_filter") == "r":
            kwargs = {k: v for k, v in kwargs.items() if k != "diff_filter"}
        return original(self, method, *args, **kwargs)

    Git._call_process = _call_process  # type: ignore[method-assign]
    Git._strixonomy_diff_filter_patch = True
