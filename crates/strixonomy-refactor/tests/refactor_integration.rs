use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strixonomy_catalog::IndexBuilder;
use strixonomy_refactor::{
    apply_refactor_plan, apply_refactor_plan_checked, find_usages, preview_cleanup_imports,
    preview_extract_module, preview_flatten_imports, preview_merge_entities,
    preview_merge_ontologies, preview_migrate_namespace, preview_move_axioms, preview_move_entity,
    preview_rename_iri, preview_replace_entity, validate_refactor_plan_paths, RefactorError,
};
use tempfile::TempDir;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/refactor")
        .canonicalize()
        .expect("fixture dir")
}

fn empty_overrides() -> HashMap<PathBuf, String> {
    HashMap::new()
}

fn workspace_roots(path: &Path) -> Vec<PathBuf> {
    vec![path.canonicalize().unwrap_or_else(|_| path.to_path_buf())]
}

fn build_catalog(dir: &std::path::Path) -> strixonomy_catalog::OntologyCatalog {
    IndexBuilder::new().workspace(dir).build().expect("index")
}

#[test]
fn find_usages_across_files() {
    let catalog = build_catalog(&fixture_dir());
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    assert!(
        usages.len() >= 2,
        "Person should be referenced in both org.ttl and people.ttl, got {:?}",
        usages
    );
    assert!(usages.iter().any(|u| u.file.ends_with("org.ttl")));
    assert!(usages.iter().any(|u| u.file.ends_with("people.ttl")));
}

#[test]
fn find_usages_rejects_person_substring_in_person_type() {
    let catalog = build_catalog(&fixture_dir());
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    assert!(!usages.iter().any(|u| u.context.contains("PersonType")));
}

#[test]
fn find_usages_finds_default_prefix_curie() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("default.ttl"),
        concat!(
            "@prefix : <http://example.org/org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n",
            ":Person a owl:Class .\n",
            ":Employee rdfs:subClassOf :Person .\n"
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    assert!(
        usages.iter().any(|u| {
            u.kind == strixonomy_refactor::UsageKind::TextReference
                && u.context.contains(":Employee")
        }),
        "expected default-prefix CURIE reference, got {:?}",
        usages
    );
}

#[test]
fn find_usages_lists_multiple_annotation_subjects() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("annotated.ttl"),
        concat!(
            "@prefix : <http://example.org/org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n",
            "@prefix skos: <http://www.w3.org/2004/02/skos/core#> .\n",
            ":Person a owl:Class ;\n",
            "    rdfs:label \"Person\" ;\n",
            "    rdfs:comment \"first\" ;\n",
            "    skos:note \"second\" .\n"
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    let annotation_subjects: Vec<_> = usages
        .iter()
        .filter(|u| u.kind == strixonomy_refactor::UsageKind::AnnotationSubject)
        .collect();
    assert!(
        annotation_subjects.len() >= 2,
        "expected multiple annotation-subject hits, got {:?}",
        annotation_subjects
    );
}

#[test]
fn find_usages_lists_multiple_annotation_objects() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("seeAlso.ttl"),
        concat!(
            "@prefix : <http://example.org/org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n",
            ":Person a owl:Class .\n",
            ":A a owl:Class ; rdfs:seeAlso :Person .\n",
            ":B a owl:Class ; rdfs:seeAlso :Person .\n",
            ":C a owl:Class ; rdfs:seeAlso :Person .\n"
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let usages = find_usages(&catalog, "http://example.org/org#Person");
    let annotation_objects: Vec<_> = usages
        .iter()
        .filter(|u| u.kind == strixonomy_refactor::UsageKind::AnnotationObject)
        .collect();
    assert!(
        annotation_objects.len() >= 3,
        "expected distinct annotation-object hits per subject, got {:?}",
        annotation_objects
    );
}

#[test]
fn rename_iri_across_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    for name in ["org.ttl", "people.ttl"] {
        std::fs::copy(fixture_dir().join(name), ws.join(name)).unwrap();
    }
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org/org#Person",
        "http://example.org/org#Human",
        &empty_overrides(),
    )
    .expect("plan");
    assert!(!plan.changes.is_empty());
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let org_text = std::fs::read_to_string(ws.join("org.ttl")).unwrap();
    assert!(org_text.contains("ex:Human"));
    assert!(!org_text.contains("ex:Person"));
    let reindexed = build_catalog(ws);
    assert!(
        reindexed.find_entity("http://example.org/org#Human").is_some(),
        "renamed IRI must appear after reindex"
    );
    assert!(
        reindexed.find_entity("http://example.org/org#Person").is_none(),
        "old IRI must be gone after reindex"
    );
}

#[test]
fn merge_entities_rewrites_references_and_removes_merged_declaration() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("merge.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:Keep a owl:Class .\n",
            "ex:Merge a owl:Class .\n",
            "ex:Child a owl:Class ; rdfs:subClassOf ex:Merge .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);

    let plan = preview_merge_entities(
        &catalog,
        "http://example.org#Keep",
        "http://example.org#Merge",
        &empty_overrides(),
    )
    .expect("merge plan");
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("ex:Keep a owl:Class"));
    assert!(preview.contains("rdfs:subClassOf ex:Keep"));
    assert!(!preview.contains("ex:Merge a owl:Class"));
    assert!(plan.warnings.is_empty());
}

#[test]
fn rename_iri_rewrites_rdf_xml() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(fixture_dir().join("person.owl"), ws.join("person.owl")).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org#Person",
        "http://example.org#Human",
        &empty_overrides(),
    )
    .expect("rename");
    assert!(plan.changes.iter().any(|c| c.path.ends_with("person.owl")));
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("http://example.org#Human"), "{preview}");
    assert!(!preview.contains("http://example.org#Person"), "{preview}");
    assert!(
        plan.warnings.iter().all(|w| !w.contains("skipping") || !w.contains("person.owl")),
        "unexpected skip warnings: {:?}",
        plan.warnings
    );
}

#[test]
fn rename_iri_rewrites_owl_xml() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(fixture_dir().join("person.owx"), ws.join("person.owx")).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org#Person",
        "http://example.org#Human",
        &empty_overrides(),
    )
    .expect("rename");
    assert!(plan.changes.iter().any(|c| c.path.ends_with("person.owx")));
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("http://example.org#Human"), "{preview}");
    assert!(!preview.contains("http://example.org#Person"), "{preview}");
}

#[test]
fn rename_iri_rewrites_obo() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(fixture_dir().join("person.obo"), ws.join("person.obo")).unwrap();
    let catalog = build_catalog(ws);
    let from = "http://purl.obolibrary.org/obo/EX_001";
    let to = "http://purl.obolibrary.org/obo/EX_010";
    assert!(
        catalog.find_entity(from).is_some(),
        "expected OBO entity in catalog; entities={:?}",
        catalog.data().entities.iter().map(|e| (&e.iri, &e.obo_id)).collect::<Vec<_>>()
    );
    let plan = preview_rename_iri(&catalog, from, to, &empty_overrides()).expect("rename");
    assert!(plan.changes.iter().any(|c| c.path.ends_with("person.obo")));
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("id: EX:010"), "{preview}");
    assert!(preview.contains("is_a: EX:010"), "{preview}");
    assert!(!preview.contains("id: EX:001"), "{preview}");
}

#[test]
fn merge_entities_rewrites_rdf_xml() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("merge.owl"),
        r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
  <owl:Class rdf:about="http://example.org#Keep">
    <rdfs:label>Keep</rdfs:label>
  </owl:Class>
  <owl:Class rdf:about="http://example.org#Merge">
    <rdfs:label>MergeLabel</rdfs:label>
    <rdfs:subClassOf rdf:resource="http://example.org#Parent"/>
  </owl:Class>
  <owl:Class rdf:about="http://example.org#Parent"/>
  <owl:Class rdf:about="http://example.org#Child">
    <rdfs:subClassOf rdf:resource="http://example.org#Merge"/>
  </owl:Class>
</rdf:RDF>
"#,
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_merge_entities(
        &catalog,
        "http://example.org#Keep",
        "http://example.org#Merge",
        &empty_overrides(),
    )
    .expect("merge");
    assert!(!plan.changes.is_empty());
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("http://example.org#Keep"), "{preview}");
    assert!(preview.contains("http://example.org#Child"), "{preview}");
    assert!(!preview.contains("http://example.org#Merge"), "{preview}");
    // Delete-then-remap (#369): merge-owned axioms must not absorb onto Keep.
    assert!(!preview.contains("MergeLabel"), "{preview}");
    assert!(
        preview.contains("rdf:resource=\"http://example.org#Keep\""),
        "Child⊑Merge must remap to Keep: {preview}"
    );
}

#[test]
fn merge_entities_ttl_and_rdf_xml_agree_on_delete_semantics() {
    let ttl = concat!(
        "@prefix ex: <http://example.org#> .\n",
        "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
        "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
        "ex:Keep a owl:Class ; rdfs:label \"Keep\" .\n",
        "ex:Merge a owl:Class ; rdfs:label \"MergeLabel\" ; rdfs:subClassOf ex:Parent .\n",
        "ex:Parent a owl:Class .\n",
        "ex:Child a owl:Class ; rdfs:subClassOf ex:Merge .\n",
    );
    let owl = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
  <owl:Class rdf:about="http://example.org#Keep">
    <rdfs:label>Keep</rdfs:label>
  </owl:Class>
  <owl:Class rdf:about="http://example.org#Merge">
    <rdfs:label>MergeLabel</rdfs:label>
    <rdfs:subClassOf rdf:resource="http://example.org#Parent"/>
  </owl:Class>
  <owl:Class rdf:about="http://example.org#Parent"/>
  <owl:Class rdf:about="http://example.org#Child">
    <rdfs:subClassOf rdf:resource="http://example.org#Merge"/>
  </owl:Class>
</rdf:RDF>
"#;

    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(ws.join("merge.ttl"), ttl).unwrap();
    std::fs::write(ws.join("merge.owl"), owl).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_merge_entities(
        &catalog,
        "http://example.org#Keep",
        "http://example.org#Merge",
        &empty_overrides(),
    )
    .expect("merge");
    assert_eq!(plan.changes.len(), 2, "expected ttl + owl changes: {:?}", plan.warnings);

    for change in &plan.changes {
        let preview = &change.preview_text;
        assert!(!preview.contains("MergeLabel"), "{}: {preview}", change.path.display());
        assert!(
            !preview.contains("http://example.org#Merge") && !preview.contains("ex:Merge"),
            "{}: merge IRI must be gone: {preview}",
            change.path.display()
        );
        // Survivor keeps its label; Child parent remapped to Keep.
        assert!(
            preview.contains("Keep") || preview.contains("\"Keep\""),
            "{}: {preview}",
            change.path.display()
        );
    }
}

#[test]
fn replace_entity_rewrites_obo_when_target_exists() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("replace.obo"),
        "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: From

[Term]
id: EX:002
name: To
is_a: EX:001 ! From
",
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let from = "http://purl.obolibrary.org/obo/EX_001";
    let to = "http://purl.obolibrary.org/obo/EX_002";
    let plan = preview_replace_entity(&catalog, from, to, &empty_overrides()).expect("replace");
    assert!(!plan.changes.is_empty());
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("is_a: EX:002"), "{preview}");
    assert!(!preview.contains("is_a: EX:001"), "{preview}");
    // Source stanza preserved; no duplicate keep id (#367).
    assert!(preview.contains("id: EX:001"), "{preview}");
    assert_eq!(
        preview.lines().filter(|l| l.trim() == "id: EX:002").count(),
        1,
        "duplicate id: EX:002: {preview}"
    );
    assert!(preview.contains("name: From"), "{preview}");
}

#[test]
fn merge_entities_removes_obo_merge_stanza_without_duplicate_ids() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("merge.obo"),
        "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Term]
id: EX:002
name: B
is_a: EX:001
",
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let keep = "http://purl.obolibrary.org/obo/EX_002";
    let merge = "http://purl.obolibrary.org/obo/EX_001";
    let plan = preview_merge_entities(&catalog, keep, merge, &empty_overrides()).expect("merge");
    assert!(!plan.changes.is_empty());
    let preview = &plan.changes[0].preview_text;
    assert!(!preview.contains("id: EX:001"), "{preview}");
    assert!(!preview.contains("name: A"), "{preview}");
    assert_eq!(
        preview.lines().filter(|l| l.trim() == "id: EX:002").count(),
        1,
        "duplicate id after merge: {preview}"
    );
    assert!(preview.contains("id: EX:002"), "{preview}");
}

#[test]
fn rename_iri_rewrites_swrl_rule_json_literals() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/swrl/basic_class_property.ttl"),
        ws.join("swrl.ttl"),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org/swrl#Person",
        "http://example.org/swrl#PersonRenamed",
        &empty_overrides(),
    )
    .expect("swrl rename");
    assert!(!plan.changes.is_empty());
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("PersonRenamed"));
    assert!(preview.contains("ontocode.dev/ns#swrlRule"));
    assert!(
        preview.contains("http://example.org/swrl#PersonRenamed"),
        "SWRL JSON should rewrite class IRI: {preview}"
    );
    assert!(!preview.contains("\"class\":\"http://example.org/swrl#Person\""));
    assert!(plan.warnings.iter().any(|w| w.contains("SWRL")));
}

#[test]
fn move_axioms_relocates_non_primary_statements() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("src.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:A a owl:Class .\n",
            "ex:A rdfs:label \"Alpha\" .\n",
            "ex:A rdfs:comment \"note\" .\n",
        ),
    )
    .unwrap();
    let target = ws.join("axioms.ttl");
    let catalog = build_catalog(ws);
    let plan = preview_move_axioms(
        &catalog,
        "http://example.org#A",
        &target,
        &[],
        true,
        &empty_overrides(),
        &[ws.to_path_buf()],
    )
    .expect("move axioms");
    assert_eq!(plan.changes.len(), 2);
    let source = plan.changes.iter().find(|c| c.path.ends_with("src.ttl")).unwrap();
    let dest = plan.changes.iter().find(|c| c.path.ends_with("axioms.ttl")).unwrap();
    assert!(source.preview_text.contains("ex:A a owl:Class"));
    assert!(!source.preview_text.contains("rdfs:label"));
    assert!(dest.preview_text.contains("rdfs:label"));
    assert!(dest.preview_text.contains("rdfs:comment"));
}

#[test]
fn merge_entities_warns_when_an_entity_is_missing() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("merge.ttl"),
        "@prefix ex: <http://example.org#> .\nex:Child ex:relatedTo ex:Missing .\n",
    )
    .unwrap();
    let catalog = build_catalog(ws);

    let plan = preview_merge_entities(
        &catalog,
        "http://example.org#Keep",
        "http://example.org#Missing",
        &empty_overrides(),
    )
    .expect("merge plan with warnings");
    assert_eq!(plan.warnings.iter().filter(|warning| warning.contains("not found")).count(), 2);
}

#[test]
fn replace_entity_preserves_source_declaration_when_target_exists() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("replace.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:From a owl:Class .\n",
            "ex:To a owl:Class .\n",
            "ex:Child a owl:Class ; rdfs:subClassOf ex:From .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);

    let plan = preview_replace_entity(
        &catalog,
        "http://example.org#From",
        "http://example.org#To",
        &empty_overrides(),
    )
    .expect("replace plan");
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("ex:From a owl:Class"));
    assert!(preview.contains("ex:To a owl:Class"));
    assert!(preview.contains("rdfs:subClassOf ex:To"));
}

#[test]
fn replace_entity_renames_declaration_when_target_is_missing() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("replace.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "ex:From a owl:Class .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);

    let plan = preview_replace_entity(
        &catalog,
        "http://example.org#From",
        "http://example.org#To",
        &empty_overrides(),
    )
    .expect("replace plan");
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("ex:To a owl:Class"));
    assert!(!preview.contains("ex:From a owl:Class"));
}

#[test]
fn migrate_namespace_updates_prefix_and_entity_iris() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org#",
        "http://example.org/v2/org#",
        &empty_overrides(),
    )
    .expect("plan");
    let preview =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("people change");
    assert!(preview.preview_text.contains("http://example.org/v2/org#"));
    assert!(preview.preview_text.contains("v2/org#"));
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let text = std::fs::read_to_string(ws.join("people.ttl")).unwrap();
    assert!(text.contains("http://example.org/v2/org#"));
    assert!(!text.contains("http://example.org/org#Person"));
    assert!(text.contains("ex:Person") || text.contains("v2"));
}

#[test]
fn migrate_namespace_preserves_slash_prefix_terminator() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    let ttl = r#"@prefix ex: <http://example.org/org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
<http://example.org/org/Person> a owl:Class .
"#;
    std::fs::write(ws.join("slash.ttl"), ttl).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org/",
        "http://example.org/v2/org/",
        &empty_overrides(),
    )
    .expect("plan");
    let preview = plan.changes.iter().find(|c| c.path.ends_with("slash.ttl")).expect("change");
    assert!(preview.preview_text.contains("<http://example.org/v2/org/>"));
    assert!(preview.preview_text.contains("@prefix ex: <http://example.org/v2/org/>"));
    assert!(!preview.preview_text.contains("<http://example.org/v2/org#>"));
}

#[test]
fn migrate_namespace_renames_multiple_angle_bracket_iris_in_one_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/org#Person> a owl:Class .
<http://example.org/org#Agent> a owl:Class .
"#;
    std::fs::write(ws.join("multi.ttl"), ttl).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org#",
        "http://example.org/v2/org#",
        &empty_overrides(),
    )
    .expect("plan");
    let preview = plan.changes.iter().find(|c| c.path.ends_with("multi.ttl")).expect("change");
    assert!(preview.preview_text.contains("<http://example.org/v2/org#Person>"));
    assert!(preview.preview_text.contains("<http://example.org/v2/org#Agent>"));
    assert!(!preview.preview_text.contains("<http://example.org/org#Person>"));
    assert!(!preview.preview_text.contains("<http://example.org/org#Agent>"));
}

#[test]
fn validate_refactor_plan_rejects_paths_outside_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path().canonicalize().unwrap();
    let outside = std::env::temp_dir().join("strixonomy-outside-refactor.ttl");
    let plan = strixonomy_refactor::RefactorPlan {
        changes: vec![strixonomy_refactor::FileChange {
            path: outside.clone(),
            preview_text: "bad".to_string(),
            original_text: String::new(),
            hunks: vec![],
        }],
        warnings: vec![],
        affected_entity_count: 0,
        affected_axiom_count: 0,
    };
    let err = validate_refactor_plan_paths(&ws, &plan).unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
    let _ = std::fs::remove_file(outside);
}

#[test]
fn move_entity_rejects_canonical_same_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let same = ws.join("./people.ttl");
    let err = preview_move_entity(
        &catalog,
        "http://example.org/org#Agent",
        &same,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
}

#[test]
fn move_entity_rejects_path_outside_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let outside = TempDir::new().unwrap();
    let target = outside.path().join("secret.ttl");
    let err = preview_move_entity(
        &catalog,
        "http://example.org/org#Agent",
        &target,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
}

#[test]
fn extract_multiple_entities_same_file() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string(), "http://example.org/org#Agent".to_string()],
        &out,
        false,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let module = std::fs::read_to_string(&out).unwrap();
    assert!(module.contains("ex:Person"));
    assert!(module.contains("ex:Agent"));
    let source = std::fs::read_to_string(ws.join("people.ttl")).unwrap();
    assert!(!source.contains("ex:Person"));
    assert!(!source.contains("ex:Agent"));
}

#[test]
fn extract_module_preserves_existing_output_content() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let out = ws.join("module.ttl");
    std::fs::write(&out, "@prefix ex: <http://example.org/org#> .\nex:Existing a owl:Class .\n")
        .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    let out_change = plan.changes.iter().find(|c| c.path == out).expect("output change");
    assert!(out_change.preview_text.contains("ex:Existing"));
    assert!(out_change.preview_text.contains("ex:Person"));
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let written = std::fs::read_to_string(&out).unwrap();
    assert!(written.contains("ex:Existing"));
    assert!(written.contains("ex:Person"));
}

#[test]
fn migrate_namespace_updates_at_prefix_uppercase_declaration() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    // Disk uses lowercase @prefix so the catalog parses; override uses @PREFIX (invalid for
    // rio, but common in the wild) to exercise replace_prefix_uri case handling.
    let path = ws.join("upper.ttl");
    let on_disk = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
"#;
    let unsaved = r#"@PREFIX ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
"#;
    std::fs::write(&path, on_disk).unwrap();
    let catalog = build_catalog(ws);
    let mut overrides = HashMap::new();
    overrides.insert(path.canonicalize().unwrap(), unsaved.to_string());
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org#",
        "http://example.org/v2/org#",
        &overrides,
    )
    .expect("plan");
    let preview = plan.changes.iter().find(|c| c.path.ends_with("upper.ttl")).expect("change");
    assert!(preview.preview_text.contains("@PREFIX ex: <http://example.org/v2/org#>"));
    assert!(!preview.preview_text.contains("@PREFIX ex: <http://example.org/org#>"));
}

#[test]
fn migrate_namespace_updates_sparql_style_prefix_declaration() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    let ttl = r#"PREFIX ex: <http://example.org/org#>
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
"#;
    std::fs::write(ws.join("sparql_prefix.ttl"), ttl).unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_migrate_namespace(
        &catalog,
        "http://example.org/org#",
        "http://example.org/v2/org#",
        &empty_overrides(),
    )
    .expect("plan");
    let preview =
        plan.changes.iter().find(|c| c.path.ends_with("sparql_prefix.ttl")).expect("change");
    assert!(preview.preview_text.contains("PREFIX ex: <http://example.org/v2/org#>"));
    assert!(!preview.preview_text.contains("PREFIX ex: <http://example.org/org#>"));
}

#[test]
fn extract_module_copies_at_prefix_uppercase_declarations() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    // Trailing @PREFIX keeps earlier triples parseable while still requiring case-insensitive
    // prefix collection for the extracted module header.
    let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .

@PREFIX other: <http://example.org/other#> .
"#;
    std::fs::write(ws.join("people.ttl"), ttl).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    let out_change = plan.changes.iter().find(|c| c.path == out).expect("output change");
    assert!(out_change.preview_text.contains("@PREFIX other: <http://example.org/other#>"));
}

#[test]
fn extract_module_stub_escapes_path_special_chars() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join(r#"mod"ule\path.ttl"#);
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        true,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    let source_change =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("source change");
    assert!(source_change.preview_text.contains(r#"mod\"ule\\path.ttl"#));
    let raw_comment = format!("Moved to {}", out.display());
    assert!(!source_change.preview_text.contains(&raw_comment));
}

#[test]
fn extract_module_leave_stub_uses_prefixed_curie() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        true,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    let source_change =
        plan.changes.iter().find(|c| c.path.ends_with("people.ttl")).expect("source change");
    assert!(source_change.preview_text.contains("ex:Person"));
    assert!(source_change.preview_text.contains("owl:deprecated true"));
    assert!(source_change.preview_text.contains("a owl:Class"));
}

#[test]
fn move_entity_between_files() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    std::fs::write(ws.join("target.ttl"), "@prefix ex: <http://example.org/org#> .\n").unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_move_entity(
        &catalog,
        "http://example.org/org#Agent",
        &ws.join("target.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    assert_eq!(plan.changes.len(), 2);
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let target = std::fs::read_to_string(ws.join("target.ttl")).unwrap();
    assert!(target.contains("ex:Agent"));
}

#[test]
fn extract_module_validates_nonexistent_output_path() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
        false,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    validate_refactor_plan_paths(ws, &plan).expect("nonexistent output path is in workspace");
    let roots = workspace_roots(ws);
    apply_refactor_plan_checked(&plan, false, Some(&roots)).expect("apply with validation");
    assert!(out.exists());
}

#[test]
fn rename_iri_renames_default_prefix_curie() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("default.ttl"),
        concat!(
            "@prefix : <http://example.org/org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            ":Person a owl:Class .\n"
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_rename_iri(
        &catalog,
        "http://example.org/org#Person",
        "http://example.org/org#Human",
        &empty_overrides(),
    )
    .expect("plan");
    assert!(!plan.changes.is_empty());
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let text = std::fs::read_to_string(ws.join("default.ttl")).unwrap();
    assert!(text.contains(":Human a owl:Class"));
    assert!(!text.contains(":Person a owl:Class"));
}

#[test]
fn move_entity_across_multi_root_workspace() {
    let tmp = TempDir::new().unwrap();
    let root_a = tmp.path().join("folder-a");
    let root_b = tmp.path().join("folder-b");
    std::fs::create_dir_all(&root_a).unwrap();
    std::fs::create_dir_all(&root_b).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), root_a.join("people.ttl")).unwrap();
    let roots =
        vec![root_a.canonicalize().expect("root a"), root_b.canonicalize().expect("root b")];
    let catalog = IndexBuilder::new()
        .workspace(roots[0].clone())
        .scan_roots(roots.clone())
        .build()
        .expect("index");
    let target = roots[1].join("moved.ttl");
    let plan = preview_move_entity(
        &catalog,
        "http://example.org/org#Person",
        &target,
        &empty_overrides(),
        &roots,
    )
    .expect("plan across secondary root");
    apply_refactor_plan_checked(&plan, false, Some(&roots)).expect("apply");
    let moved = std::fs::read_to_string(&target).expect("target file");
    assert!(moved.contains("ex:Person"));
    let source = std::fs::read_to_string(root_a.join("people.ttl")).unwrap();
    assert!(!source.contains("ex:Person"));
}

#[test]
fn move_entity_to_new_file_includes_prefix_declarations() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let target = ws.join("moved.ttl");
    let plan = preview_move_entity(
        &catalog,
        "http://example.org/org#Person",
        &target,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let moved = std::fs::read_to_string(&target).expect("target");
    assert!(
        moved.lines().any(|l| l.trim_start().to_ascii_lowercase().starts_with("@prefix")),
        "new target must include @prefix lines: {moved}"
    );
    assert!(moved.contains("ex:Person"));
}

#[test]
fn move_entity_into_nonempty_target_merges_missing_prefixes() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::create_dir_all(ws).unwrap();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    // Target has owl but not ex — the moved block uses ex: CURIE (#314).
    std::fs::write(ws.join("target.ttl"), "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n")
        .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_move_entity(
        &catalog,
        "http://example.org/org#Person",
        &ws.join("target.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("plan");
    apply_refactor_plan(&plan, false, ws).expect("apply");
    let target = std::fs::read_to_string(ws.join("target.ttl")).unwrap();
    assert!(
        target.lines().any(|l| l.contains("@prefix ex:") || l.contains("@prefix ex :")),
        "non-empty target must gain missing source prefixes: {target}"
    );
    assert!(target.contains("ex:Person"));
}

#[test]
fn merge_ontologies_unions_prefixes_and_appends_body() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("target.ttl"),
        concat!(
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix ex: <http://example.org#> .\n\n",
            "ex:Keep a owl:Class .\n",
        ),
    )
    .unwrap();
    std::fs::write(
        ws.join("source.ttl"),
        concat!(
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix ex: <http://example.org#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:Added a owl:Class ;\n",
            "    rdfs:label \"Added\" .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_merge_ontologies(
        &catalog,
        &[ws.join("source.ttl")],
        &ws.join("target.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("merge plan");
    assert_eq!(plan.changes.len(), 1);
    let preview = &plan.changes[0].preview_text;
    assert!(preview.contains("ex:Keep"));
    assert!(preview.contains("ex:Added"));
    assert!(preview.contains("@prefix rdfs:"));
    apply_refactor_plan(&plan, false, ws).expect("apply");
}

#[test]
fn merge_ontologies_rejects_path_outside_workspace() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(ws.join("a.ttl"), "@prefix ex: <http://example.org#> .\nex:A a owl:Class .\n")
        .unwrap();
    let catalog = build_catalog(ws);
    let outside = TempDir::new().unwrap();
    let err = preview_merge_ontologies(
        &catalog,
        &[ws.join("a.ttl")],
        &outside.path().join("out.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .unwrap_err();
    assert!(matches!(err, RefactorError::Invalid(_)));
}

#[test]
fn flatten_imports_inlines_imported_axioms_and_removes_imports() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("lib.ttl"),
        concat!(
            "@prefix ex: <http://example.org/lib#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
            "<http://example.org/lib> a owl:Ontology .\n",
            "ex:LibClass a owl:Class .\n",
        ),
    )
    .unwrap();
    std::fs::write(
        ws.join("root.ttl"),
        concat!(
            "@prefix ex: <http://example.org/root#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
            "<http://example.org/root> a owl:Ontology ;\n",
            "    owl:imports <http://example.org/lib> .\n",
            "ex:RootClass a owl:Class .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_flatten_imports(
        &catalog,
        &ws.join("root.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("flatten plan");
    assert_eq!(plan.changes.len(), 1);
    let preview = &plan.changes[0].preview_text;
    assert!(!preview.contains("owl:imports"));
    assert!(preview.contains("ex:LibClass") || preview.contains("LibClass"));
    assert!(preview.contains("ex:RootClass"));
}

#[test]
fn cleanup_imports_removes_unused_import() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("lib.ttl"),
        concat!(
            "@prefix ex: <http://example.org/lib#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
            "<http://example.org/lib> a owl:Ontology .\n",
            "ex:UnusedLib a owl:Class .\n",
        ),
    )
    .unwrap();
    std::fs::write(
        ws.join("root.ttl"),
        concat!(
            "@prefix ex: <http://example.org/root#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
            "<http://example.org/root> a owl:Ontology ;\n",
            "    owl:imports <http://example.org/lib> .\n",
            "ex:Local a owl:Class .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_cleanup_imports(
        &catalog,
        &ws.join("root.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("cleanup plan");
    assert_eq!(plan.changes.len(), 1);
    let preview = &plan.changes[0].preview_text;
    assert!(!preview.contains("owl:imports"));
    assert!(preview.contains("ex:Local"));
}

#[test]
fn cleanup_imports_keeps_used_import() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("lib.ttl"),
        concat!(
            "@prefix lib: <http://example.org/lib#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
            "<http://example.org/lib> a owl:Ontology .\n",
            "lib:Shared a owl:Class .\n",
        ),
    )
    .unwrap();
    std::fs::write(
        ws.join("root.ttl"),
        concat!(
            "@prefix root: <http://example.org/root#> .\n",
            "@prefix lib: <http://example.org/lib#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "<http://example.org/root> a owl:Ontology ;\n",
            "    owl:imports <http://example.org/lib> .\n",
            "root:Child a owl:Class ; rdfs:subClassOf lib:Shared .\n",
        ),
    )
    .unwrap();
    let catalog = build_catalog(ws);
    let plan = preview_cleanup_imports(
        &catalog,
        &ws.join("root.ttl"),
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("cleanup plan");
    assert!(
        plan.changes.is_empty(),
        "used import must be retained, got {:?}",
        plan.changes.first().map(|c| &c.preview_text)
    );
}

#[test]
fn extract_module_locality_expands_related_entities() {
    let tmp = TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::copy(fixture_dir().join("people.ttl"), ws.join("people.ttl")).unwrap();
    let catalog = build_catalog(ws);
    let out = ws.join("module.ttl");
    // Person ⊑ Agent — locality should pull Agent into the module.
    let plan = preview_extract_module(
        &catalog,
        &["http://example.org/org#Person".to_string()],
        &out,
        false,
        true,
        &empty_overrides(),
        &workspace_roots(ws),
    )
    .expect("locality extract");
    assert!(plan.warnings.iter().any(|w| w.contains("locality")));
    let module = plan.changes.iter().find(|c| c.path == out).expect("module change");
    assert!(module.preview_text.contains("ex:Person"));
    assert!(
        module.preview_text.contains("ex:Agent"),
        "locality should expand Person→Agent: {}",
        module.preview_text
    );
}
