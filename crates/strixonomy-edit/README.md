# strixonomy-edit

Canonical **semantic transaction** layer for Strixonomy ontology editing (v0.19+).

All Turtle and OBO apply paths should route through `Transaction` rather than calling
format-specific patch engines directly from product code.

## Quick start

```rust
use strixonomy_edit::{SemanticChange, Transaction};
use strixonomy_owl::PatchOp;

let txn = Transaction::from_turtle(vec![PatchOp::SetLabel {
    entity_iri: "http://ex#A".into(),
    value: "Example".into(),
}]);

let inverted = txn.invert()?;
```

See [ADR-0020](../../docs/design/adr/0020-semantic-transaction-edit-model.md).
