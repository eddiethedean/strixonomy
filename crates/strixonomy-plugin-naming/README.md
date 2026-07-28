# strixonomy-plugin-naming

Reference **naming / label** validator for the Strixonomy plugin host.

| | |
|--|--|
| Plugin id | `strixonomy.naming-validator` |
| Kind | validator |
| Permission | `workspace.read` |

## What it does

Emits diagnostics when classes/properties are missing `rdfs:label` or IRIs fail a configured prefix check. Invoked during `strixonomy validate` / workspace index when the plugin is listed under `.strixonomy/plugins/*.toml`.

## Try it

1. Copy a TOML manifest from [examples/plugin-workspace](https://github.com/eddiethedean/strixonomy/tree/main/examples/plugin-workspace).
2. Run `strixonomy validate .` or open the folder in Strixonomy.

## Authoring docs

Canonical contract: [Plugin authoring](https://strixonomy.readthedocs.io/en/latest/guides/plugins/). Do **not** implement against historical trait specs or the future OntoUI TypeScript Plugin API.

## License

MIT OR Apache-2.0
