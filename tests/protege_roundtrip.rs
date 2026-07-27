//! Protégé-style round-trip tests (OWL_AUTHORING_SPEC §5) + v0.18 workflow fixtures.

use std::collections::BTreeMap;
use std::path::PathBuf;
use strixonomy::Workspace;
use strixonomy_catalog::IndexBuilder;
use strixonomy_core::DiagnosticCode;
use strixonomy_owl::{apply_patches, apply_patches_to_text, PatchOp};
use strixonomy_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

fn roundtrip_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/protege-roundtrip")
}

#[test]
fn protege_roundtrip_properties_domain_range() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path();
    let path = workspace.join("properties.ttl");
    std::fs::copy(roundtrip_dir().join("properties.ttl"), &path).expect("copy properties.ttl");

    let ns = BTreeMap::from([
        ("ex".to_string(), "http://example.org/props#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string()),
    ]);
    let has_age = "http://example.org/props#hasAge";
    let person = "http://example.org/props#Person";
    let xsd_integer = "http://www.w3.org/2001/XMLSchema#integer";

    let preview = apply_patches_to_text(
        &std::fs::read_to_string(&path).expect("read"),
        &[PatchOp::AddComment {
            entity_iri: has_age.to_string(),
            value: "Age in years".to_string(),
        }],
        true,
        &ns,
    )
    .expect("preview patch");
    let preview_text = preview.preview_text.expect("preview text");
    assert!(
        preview_text.contains("Age in years"),
        "comment value must appear in preview: {preview_text}"
    );

    apply_patches(
        &path,
        &[PatchOp::AddComment {
            entity_iri: has_age.to_string(),
            value: "Age in years".to_string(),
        }],
        false,
        &ns,
    )
    .expect("apply comment");

    let catalog = IndexBuilder::new().workspace(workspace).build().expect("reindex");
    let detail = catalog.entity_detail(has_age).expect("hasAge detail");
    assert!(
        detail.axioms.iter().any(|a| {
            a.kind == "domain" && (a.display.contains(person) || a.display.contains("Person"))
        }),
        "expected domain Person, got {:?}",
        detail.axioms
    );
    assert!(
        detail.axioms.iter().any(|a| {
            a.kind == "range" && (a.display.contains(xsd_integer) || a.display.contains("integer"))
        }),
        "expected range xsd:integer, got {:?}",
        detail.axioms
    );
    assert!(detail.characteristics.functional, "hasAge is declared owl:FunctionalProperty");
    assert!(
        detail.annotations.iter().any(|a| a.value.contains("Age in years"))
            || std::fs::read_to_string(&path).unwrap().contains("Age in years"),
        "comment must persist after write-back"
    );
}

#[test]
fn protege_roundtrip_individuals_indexed() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws.catalog().entity_detail("http://example.org/people#Alice").expect("Alice");
    assert!(detail.axioms.iter().any(|a| a.kind == "class_assertion"));
    assert!(detail.axioms.iter().any(|a| a.kind == "object_property_assertion"));
}

#[test]
fn protege_roundtrip_people_classes_and_labels() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let person = ws.catalog().entity_detail("http://example.org/people#Person").expect("Person");
    assert_eq!(person.entity.short_name, "Person");
    assert!(
        !person.entity.labels.is_empty(),
        "expected Person label, got {:?}",
        person.entity.labels
    );
    assert!(ws.catalog().find_entity("http://example.org/people#worksFor").is_some());
}

#[test]
fn protege_roundtrip_property_chains_indexed() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws
        .catalog()
        .entity_detail("http://example.org/chains#hasGrandparent")
        .expect("hasGrandparent");
    assert!(
        detail.axioms.iter().any(|a| a.kind == "property_chain"),
        "expected property_chain axiom, got {:?}",
        detail.axioms.iter().map(|a| &a.kind).collect::<Vec<_>>()
    );
}

#[test]
fn protege_roundtrip_annotations_indexed() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws.catalog().entity_detail("http://example.org/ann#Concept").expect("Concept");
    assert!(
        detail.annotations.iter().any(|a| {
            a.predicate.contains("definition") || a.value.contains("annotated example")
        }),
        "expected skos:definition annotation on Concept, got {:?}",
        detail.annotations
    );
    assert!(
        detail.annotations.iter().any(|a| {
            a.predicate.ends_with("seeAlso") || a.value.contains("http://example.org/docs")
        }),
        "expected seeAlso annotation, got {:?}",
        detail.annotations
    );
    assert!(ws.catalog().find_entity("http://example.org/ann#seeAlso").is_some());
}

#[test]
fn protege_roundtrip_owl_rdfxml_horned() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    assert!(ws.catalog().find_entity("http://example.org/org#Department").is_some());
}

#[test]
fn protege_roundtrip_owl_rdfxml_edit_label_reload() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path();
    let path = workspace.join("organization.owl");
    std::fs::copy(roundtrip_dir().join("organization.owl"), &path).expect("copy");

    let dept = "http://example.org/org#Department";
    let ns = BTreeMap::new();
    apply_patches(
        &path,
        &[PatchOp::SetLabel { entity_iri: dept.to_string(), value: "Dept Renamed".to_string() }],
        false,
        &ns,
    )
    .expect("apply SetLabel to RDF/XML");

    let text = std::fs::read_to_string(&path).expect("read");
    let (ont, incomplete) = strixonomy_owl::load_rdf_xml_ontology(&text).expect("reload");
    assert!(!incomplete);
    let bridge = strixonomy_owl::bridge_ontology(ont, "doc", &text, &ns);
    let entity = bridge.entities.iter().find(|e| e.iri == dept).expect("Department");
    assert!(
        entity.labels.iter().any(|l| l == "Dept Renamed"),
        "expected renamed label, got {:?}",
        entity.labels
    );
    assert!(
        bridge.imports.iter().any(|i| i.import_iri.contains("people")),
        "imports must survive RDF/XML write-back: {:?}",
        bridge.imports
    );
}

#[test]
fn protege_roundtrip_owx_edit_parent_reload() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path();
    let path = workspace.join("example.owx");
    std::fs::copy(roundtrip_dir().join("example.owx"), &path).expect("copy");

    let dept = "http://example.org/org#Department";
    let old_parent = "http://example.org/people#Organization";
    let new_parent = "http://example.org/people#Person";
    let ns = BTreeMap::new();
    apply_patches(
        &path,
        &[
            PatchOp::RemoveSubClassOf {
                entity_iri: dept.to_string(),
                parent_iri: old_parent.to_string(),
            },
            PatchOp::AddSubClassOf {
                entity_iri: dept.to_string(),
                parent_iri: new_parent.to_string(),
            },
        ],
        false,
        &ns,
    )
    .expect("apply parent change to OWL/XML");

    let text = std::fs::read_to_string(&path).expect("read");
    let (ont, _, _) = strixonomy_owl::load_owl_xml_ontology(&text).expect("reload");
    let bridge = strixonomy_owl::bridge_ontology(ont, "doc", &text, &ns);
    assert!(bridge.entities.iter().any(|e| e.iri == dept));
    assert!(
        bridge.axioms.iter().any(|a| {
            a.axiom_kind == strixonomy_core::AXIOM_KIND_SUB_CLASS_OF
                && a.subject == dept
                && a.object == new_parent
        }),
        "expected new parent, axioms={:?}",
        bridge.axioms
    );
}

#[test]
fn protege_rdfxml_malformed_refuses_writeback() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("broken.owl");
    std::fs::write(&path, "<rdf:RDF>not valid").expect("write");
    let err = apply_patches(
        &path,
        &[PatchOp::SetLabel { entity_iri: "http://example.org/x".into(), value: "x".into() }],
        false,
        &BTreeMap::new(),
    );
    assert!(err.is_err(), "malformed RDF/XML must fail closed");
    let remaining = std::fs::read_to_string(&path).expect("read");
    assert_eq!(remaining, "<rdf:RDF>not valid", "file must not be truncated");
}

#[test]
fn protege_xml_unsupported_op_leaves_file_unchanged() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("organization.owl");
    std::fs::copy(roundtrip_dir().join("organization.owl"), &path).expect("copy");
    let before = std::fs::read_to_string(&path).expect("read");
    let err = apply_patches(
        &path,
        &[PatchOp::AddPrefix {
            prefix: "ex".into(),
            namespace_iri: "http://example.org/x#".into(),
        }],
        false,
        &BTreeMap::new(),
    );
    assert!(err.is_err());
    let after = std::fs::read_to_string(&path).expect("read");
    assert_eq!(before, after, "unsupported op must not rewrite the file");
}

#[test]
fn protege_semantic_compare_rdfxml_roundtrip_equal() {
    let src = std::fs::read_to_string(roundtrip_dir().join("organization.owl")).expect("read");
    let (left, _) = strixonomy_owl::load_rdf_xml_ontology(&src).expect("load");
    let out = strixonomy_owl::serialize_rdf_xml(&left).expect("serialize");
    let (right, _) = strixonomy_owl::load_rdf_xml_ontology(&out).expect("reload");
    let diff = strixonomy_owl::compare_ontologies(left, right);
    assert!(diff.is_empty(), "semantic round-trip diff: {diff:?}");
}

#[test]
fn protege_roundtrip_owx_horned() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let detail = ws
        .catalog()
        .entity_detail("http://example.org/org#Department")
        .expect("Department from example.owx");
    assert_eq!(detail.entity.short_name, "Department");
}

#[test]
fn protege_workflow_classify_people_el() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path();
    std::fs::copy(roundtrip_dir().join("people.ttl"), workspace.join("people.ttl"))
        .expect("copy people.ttl");
    let input = WorkspaceInputLoader::new(workspace).load().expect("load reasoner input");
    let result = classify(ReasonerId::El, &input, false).expect("classify");
    assert!(result.consistent, "people fixture should be consistent");
    assert_eq!(result.profile_used, "el");
    // Hierarchy oracle: at least one asserted class edge must survive classification.
    assert!(
        !result.inferred.combined.edges.is_empty() || input.asserted_hierarchy.edges.is_empty(),
        "combined hierarchy should preserve asserted subclass edges when present"
    );
}

#[test]
fn protege_workflow_broken_import_fixture_diagnosed() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/diagnostics");
    let broken = fixtures.join("lint-broken-import.ttl");
    assert!(broken.exists(), "missing fixture {}; test must not silently skip", broken.display());
    let ws = Workspace::open(&fixtures).expect("open diagnostics fixtures");
    let diags = ws.diagnostics();
    assert!(
        diags.iter().any(|d| matches!(d.code, DiagnosticCode::BrokenImport)),
        "expected DiagnosticCode::BrokenImport, got {:?}",
        diags.iter().map(|d| (&d.code, &d.message)).collect::<Vec<_>>()
    );
}
