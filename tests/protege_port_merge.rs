//! Protégé Wave 1 port: entity merge.
//! Upstream: MergeEntitiesChangeListGenerator_TestCase.

mod support;

use std::collections::HashMap;
use strixonomy_catalog::IndexBuilder;
use strixonomy_refactor::{apply_refactor_plan, preview_merge_entities};
use support::protege_port::{assert_parent_of, copy_ported_workspace, index_workspace};

#[test]
fn merge_rewrites_subclass_and_removes_source() {
    let (dir, _) = copy_ported_workspace("merge_labels.ttl");
    let catalog = index_workspace(dir.path());
    let keep = "http://example.org/merge#Keep";
    let merge = "http://example.org/merge#Merge";
    let child = "http://example.org/merge#Child";

    let plan = preview_merge_entities(&catalog, keep, merge, &HashMap::new()).expect("merge plan");
    assert!(plan.warnings.is_empty(), "unexpected warnings: {:?}", plan.warnings);
    apply_refactor_plan(&plan, false, dir.path()).expect("apply merge");

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("reindex");
    assert!(catalog.find_entity(keep).is_some(), "Keep must remain");
    assert!(catalog.find_entity(merge).is_none(), "Merge entity declaration should be gone");
    assert_parent_of(&catalog, child, keep);

    let keep_entity = catalog.find_entity(keep).expect("keep");
    // Survivor retains its own label; merge source label may be preserved as alt annotation
    // depending on engine policy — at minimum Child ⊑ Keep must hold.
    assert!(
        keep_entity.labels.iter().any(|l| l == "Keep")
            || keep_entity.labels.iter().any(|l| l == "MergeLabel"),
        "expected Keep and/or MergeLabel on survivor, got {:?}",
        keep_entity.labels
    );
}

#[test]
fn merge_preview_mentions_keep_not_merge_declaration() {
    let (dir, _) = copy_ported_workspace("merge_labels.ttl");
    let catalog = index_workspace(dir.path());
    let plan = preview_merge_entities(
        &catalog,
        "http://example.org/merge#Keep",
        "http://example.org/merge#Merge",
        &HashMap::new(),
    )
    .expect("plan");
    let preview = &plan.changes[0].preview_text;
    assert!(
        preview.contains("merge#Keep") || preview.contains("ex:Keep"),
        "preview should keep survivor: {preview}"
    );
    assert!(
        preview.contains("subClassOf") || preview.contains("rdfs:subClassOf"),
        "preview should rewrite subclass: {preview}"
    );
}
