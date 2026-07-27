# ROBOT interop

Strixonomy v0.19.0 wraps the [ROBOT](http://robot.obolibrary.org/) CLI for validate, merge, and report workflows. ROBOT runs as an **external Java process** — Strixonomy does not embed a JVM.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Prerequisites

| Requirement | Notes |
|-------------|-------|
| **Java** | ROBOT requires a JRE on the agent or developer machine |
| **`robot` on PATH** | Or set an explicit path (see below) |
| **Strixonomy 0.19.0+** | `strixonomy robot` subcommand or LSP `strixonomy/runRobot` |

Install ROBOT from [robot.obolibrary.org](http://robot.obolibrary.org/).

## CLI

```bash
# Validate
strixonomy robot validate path/to/ontology.owl

# Merge
strixonomy robot merge --inputs a.owl --inputs b.owl --output merged.owl

# Report
strixonomy robot report path/to/ontology.owl --report report.tsv
```

Override the executable:

```bash
strixonomy robot validate demo.obo --robot-path /opt/robot/robot.jar
```

Full flag reference: [CLI reference](../cli-reference.md#robot).

## VS Code

Set **`strixonomy.robotPath`** in settings to the `robot` executable or JAR when it is not on `PATH`. Trusted workspaces only (ignored in Restricted Mode).

LSP clients can call `strixonomy/runRobot` — see [LSP API](../lsp-api.md).

## CI recipe

```yaml
- name: Install Strixonomy
  run: cargo install strixonomy-cli --locked --version 0.27.0

- name: Strixonomy validate
  run: strixonomy validate ./ontologies

- name: ROBOT validate
  run: strixonomy robot validate ./ontologies/core.owl
```

Example with OBO fixtures: [`examples/obo-workflow/`](https://github.com/eddiethedean/strixonomy/tree/main/examples/obo-workflow).

## Security note

ROBOT spawns a child process with user-supplied arguments. Run in trusted CI agents only; do not expose `runRobot` over an unauthenticated network LSP bridge.

## Related

- [OBO workflow guide](obo-workflow.md)
- [CI integration](../ci-integration.md)
- [Enterprise evaluation](enterprise-eval.md)
