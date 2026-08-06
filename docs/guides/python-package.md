# Package status page for the reserved `strixonomy` PyPI distribution.

# Python package (PyPI reservation)

The official **`strixonomy`** project on [PyPI](https://pypi.org/project/strixonomy/) is a **name reservation** artifact shipped in **v0.28.1**. It does **not** ship the Python SDK.

| Artifact | Status in v0.28 | Capabilities |
|----------|-----------------|--------------|
| **PyPI `strixonomy`** | Reserved pre-release | Version metadata + `status()` only — **no** workspace/query/validation APIs |
| **CLI `strixonomy`** | Shipped | Index, query, patch, refactor, validate, plugins |
| **LSP `strixonomy-lsp`** | Shipped | Editor integration, custom `strixonomy/*` methods |
| **Rust `strixonomy` / `strixonomy-*`** | Shipped | Library and plugin SDK |
| **VS Code extension** | Shipped | Full IDE surface |
| **Python SDK (PyO3/Maturin)** | Planned **v0.34** | Stable Python APIs over the Rust engine |

## Why reserve the name?

Before v0.31, the project publishes an honest reservation wheel so:

- The official maintainers control the `strixonomy` PyPI identity
- The official maintainers control the `strixonomy` PyPI identity via the release workflow
- Users and integrators are not misled by unofficial `strixonomy` packages claiming engine capabilities

## Install the reservation package (optional)

```bash
pip install strixonomy
```

```python
import strixonomy

strixonomy.__version__  # e.g. "0.28.1"
strixonomy.status()
# {'package': 'strixonomy', 'version': '0.28.1', 'reservation': True,
#  'sdk_planned_release': '1.1', ...}
```

Importing `strixonomy` does **not** load the Rust engine or provide ontology APIs.

## What to use instead (today)

| Goal | Use |
|------|-----|
| Edit ontologies in VS Code | [Install the extension](../vscode-install.md) |
| Automation / CI | [CLI](../cli-reference.md) — `cargo install strixonomy-cli --locked --version 0.28.1` |
| Custom editor | [LSP](../lsp-api.md) |
| Rust integration | [Rust library](rust-library.md) |

See [Which artifact](../guides/which-artifact.md) and [API parity matrix](../reference/api-parity-matrix.md).

## Package layout (repository)

Source lives under [`python/`](https://github.com/eddiethedean/strixonomy/tree/main/python) in the monorepo:

```
python/
  pyproject.toml      # hatchling build (reservation wheel)
  README.md           # PyPI long description
  src/strixonomy/     # pure-Python reservation module
```

v0.31 will add Maturin/PyO3 metadata and native bindings without changing the PyPI project name.

## Release and security

- **Publishing:** GitHub Release tags trigger PyPI upload from `.github/workflows/release.yml` using the `PYPI_API_TOKEN` repository secret (same tag gate as crates.io).
- **Maintainers:** PyPI project owners must use **2FA**.
- **Procedure:** [Releasing](../releasing.md) — Python section.

## Non-goals (v0.28)

- Python bindings to `strixonomy-core`
- Subprocess wrappers marketed as an SDK
- Stable Python public API
- Production support for `pip install strixonomy` beyond name reservation

Those are **v0.31** deliverables on the [roadmap](../roadmap.md).

## Related

- [Migration v0.27 → v0.28](../migration/v0.28.md) — compat shim removal (separate from PyPI reservation)
- [SHIPPED](../SHIPPED.md) — canonical capability matrix
- [Rust crates guide](rust-crates.md)
