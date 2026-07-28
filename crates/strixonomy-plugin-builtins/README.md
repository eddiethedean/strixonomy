# strixonomy-plugin-builtins

Aggregates **built-in reference plugins** shipped with Strixonomy (naming, markdown export, SHACL scaffold, and related fixtures).

## Audience

Contributors and plugin authors who want working reference implementations next to the host.

- End users: enable plugins via `.strixonomy/plugins/*.toml` — [Plugin authoring](https://strixonomy.readthedocs.io/en/latest/guides/plugins/)
- Individual crates: `strixonomy-plugin-naming`, `strixonomy-plugin-markdown-export`, `strixonomy-plugin-shacl`

## Status

Plugin host and reference plugins are **MVP / experimental** relative to a v1.0 stable ecosystem API. Permissions (`api_version = "1"`) are enforced; do not assume marketplace stability yet.

## License

MIT OR Apache-2.0
