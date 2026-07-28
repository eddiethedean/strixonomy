# strixonomy (PyPI reservation)

This is the **official reserved** `strixonomy` distribution on PyPI. It secures the package name and establishes an automated release path for the future Python SDK.

## What this package is

- A **name reservation** artifact published by the Strixonomy project
- Version metadata and a `status()` helper that describe SDK plans

## What this package is not

This distribution does **not** provide:

- Ontology workspace APIs
- Query, validation, diff, or reasoning bindings
- CLI subprocess wrappers presented as an SDK
- Stable Python APIs or production support

For shipped capabilities today, use:

| Need | Artifact |
|------|----------|
| VS Code IDE | [Strixonomy extension](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) |
| CLI / automation | `cargo install strixonomy-cli --locked` |
| Editor integration | `strixonomy-lsp` |
| Rust library | `strixonomy` on [crates.io](https://crates.io/crates/strixonomy) |

## Python SDK timeline

A real Python SDK with PyO3/Maturin bindings is planned for **Strixonomy v1.1**. See [Python package status](https://strixonomy.readthedocs.io/en/latest/guides/python-package/) and the [roadmap](https://strixonomy.readthedocs.io/en/latest/roadmap/).

## Install (optional)

```bash
pip install strixonomy
```

```python
import strixonomy

print(strixonomy.__version__)
print(strixonomy.status())
```

## License

MIT OR Apache-2.0 (same as the Strixonomy engine).
