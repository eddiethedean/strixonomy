use strixonomy_catalog::IndexBuilder;
use strixonomy_core::AXIOM_KIND_SUB_CLASS_OF;
use strixonomy_diff::{diff_catalogs, BreakingReason, EntityChangeKind};

#[test]
fn removed_subclass_emits_breaking() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\nex:B a owl:Class .\nex:A rdfs:subClassOf ex:B .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\nex:B a owl:Class .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    let breaking: Vec<_> = diff
        .breaking_changes
        .iter()
        .filter(|b| matches!(b.reason, BreakingReason::RemovedSuperclass))
        .collect();
    assert_eq!(
        breaking.len(),
        1,
        "expected exactly one RemovedSuperclass breaking change, got {:?}",
        diff.breaking_changes
    );
    assert!(
        breaking[0].entity_iri.as_deref() == Some("http://ex/A")
            || breaking[0].message.contains("A"),
        "breaking change should reference class A: {:?}",
        breaking[0]
    );
    assert!(
        diff.axiom_changes.iter().any(|a| {
            a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF
                && a.change == "removed"
                && a.subject.contains("A")
        }),
        "subclass axiom removal must appear in axiom_changes: {:?}",
        diff.axiom_changes
    );
}

#[test]
fn new_deprecation_reported() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class ; owl:deprecated true .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    assert!(
        diff.entity_changes.iter().any(|c| c.kind == EntityChangeKind::Deprecated),
        "expected deprecation change"
    );
}

#[test]
fn shared_label_alone_is_not_a_rename() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
         ex:Old a owl:Class ; rdfs:label \"SharedLabel\" .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
         ex:New a owl:Class ; rdfs:label \"SharedLabel\" .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    assert!(
        !diff.entity_changes.iter().any(|c| c.kind == EntityChangeKind::Renamed),
        "shared labels must not be treated as renames: {:?}",
        diff.entity_changes
    );
    assert!(
        diff.entity_changes.iter().any(|c| c.kind == EntityChangeKind::Removed),
        "expected removed Old"
    );
    assert!(
        diff.entity_changes.iter().any(|c| c.kind == EntityChangeKind::Added),
        "expected added New"
    );
}

#[test]
fn owl_same_as_links_renamed_entities() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         ex:Old a owl:NamedIndividual, owl:Class .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         ex:New a owl:NamedIndividual, owl:Class ;\n\
           owl:sameAs ex:Old .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    assert!(
        head.data().annotations.iter().any(|a| {
            a.predicate.contains("sameAs")
                && ((a.subject.contains("New") && a.object.contains("Old"))
                    || (a.subject.contains("Old") && a.object.contains("New")))
        }),
        "head catalog must materialize owl:sameAs annotations; got {:?}",
        head.data()
            .annotations
            .iter()
            .map(|a| format!("{} {} {}", a.subject, a.predicate, a.object))
            .collect::<Vec<_>>()
    );
    let diff = diff_catalogs(&base, &head);
    assert!(
        diff.entity_changes.iter().any(|c| {
            c.kind == EntityChangeKind::Renamed
                && c.iri.contains("New")
                && c.previous_iri.as_deref().is_some_and(|p| p.contains("Old"))
        }),
        "sameAs should produce Renamed: {:?}",
        diff.entity_changes
    );
    assert!(
        !diff
            .entity_changes
            .iter()
            .any(|c| { c.kind == EntityChangeKind::Removed && c.iri.contains("Old") }),
        "renamed Old must not remain as Removed: {:?}",
        diff.entity_changes
    );
}

#[test]
fn removed_same_as_does_not_count_as_rename() {
    let dir = tempfile::tempdir().unwrap();
    let base_path = dir.path().join("base.ttl");
    std::fs::write(
        &base_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         ex:Old a owl:NamedIndividual, owl:Class ;\n\
           owl:sameAs ex:New .\n\
         ex:New a owl:NamedIndividual, owl:Class .\n",
    )
    .unwrap();
    let head_path = dir.path().join("head.ttl");
    std::fs::write(
        &head_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         ex:New a owl:NamedIndividual, owl:Class .\n",
    )
    .unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![base_path])
        .build()
        .expect("base");
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![head_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    assert!(
        !diff.entity_changes.iter().any(|c| c.kind == EntityChangeKind::Renamed),
        "removed sameAs must not invent Renamed: {:?}",
        diff.entity_changes
    );
}

#[test]
fn axiom_diff_is_per_ontology_id() {
    let dir = tempfile::tempdir().unwrap();
    let a_path = dir.path().join("a.ttl");
    let b_path = dir.path().join("b.ttl");
    let shared = "@prefix ex: <http://ex/> .\n\
         @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         ex:A a owl:Class .\n\
         ex:B a owl:Class .\n\
         ex:A rdfs:subClassOf ex:B .\n";
    std::fs::write(&a_path, shared).unwrap();
    std::fs::write(&b_path, shared).unwrap();
    let base = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![a_path.clone(), b_path.clone()])
        .build()
        .expect("base");
    // Head keeps the axiom only in b.ttl.
    std::fs::write(
        &a_path,
        "@prefix ex: <http://ex/> .\n\
         @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
         ex:A a owl:Class .\n\
         ex:B a owl:Class .\n",
    )
    .unwrap();
    let head = IndexBuilder::new()
        .workspace(dir.path())
        .only_paths(vec![a_path, b_path])
        .build()
        .expect("head");
    let diff = diff_catalogs(&base, &head);
    assert!(
        diff.axiom_changes.iter().any(|a| {
            a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF
                && a.change == "removed"
                && a.subject.contains("A")
        }),
        "per-document axiom removal must appear: {:?}",
        diff.axiom_changes
    );
    assert!(
        diff.breaking_changes.iter().any(|b| matches!(b.reason, BreakingReason::RemovedSuperclass)),
        "expected RemovedSuperclass breaking change: {:?}",
        diff.breaking_changes
    );
}
