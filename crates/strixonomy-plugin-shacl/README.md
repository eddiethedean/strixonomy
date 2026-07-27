# strixonomy-plugin-shacl

Reference **SHACL** validator scaffold for the Strixonomy plugin host.

| | |
|--|--|
| Plugin id | `strixonomy.shacl-validator` |
| Kind | validator |
| Permission | `workspace.read` |

## What it does

Looks for a shapes directory (`shapes_dir` in the plugin config) and emits informational diagnostics when shapes are missing, empty, or pending full validation. Full rudof-backed validation is incomplete — treat this as a scaffold, not production SHACL CI.

## Try it

1. Add a plugin TOML pointing at this binary (see [examples/plugin-workspace](https://github.com/eddiethedean/strixonomy/tree/main/examples/plugin-workspace)).
2. Run `strixonomy validate .`.

## Authoring docs

[Plugin authoring](https://strixonomy-vs.readthedocs.io/en/latest/guides/plugins/) — TOML + subprocess JSON only.

## License

MIT OR Apache-2.0
