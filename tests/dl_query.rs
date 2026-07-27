use std::collections::BTreeMap;
use std::path::PathBuf;
use strixonomy_reasoner::{run_dl_query, DlQueryMode, ReasonerId, WorkspaceInputLoader};

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[test]
fn dl_query_named_class_returns_hierarchy() {
    let input = WorkspaceInputLoader::new(fixture_dir()).load().expect("load fixtures");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
    let result =
        run_dl_query(ReasonerId::Rl, &input, "ex:ClinicPerson", &namespaces, DlQueryMode::Inferred)
            .expect("dl query");
    assert!(!result.subclasses.is_empty(), "expected subclasses of ClinicPerson");
    assert!(
        result.subclasses.iter().any(|iri| iri.contains("Patient") || iri.contains("Staff")),
        "subclasses={:?}",
        result.subclasses
    );
}

#[test]
fn dl_query_parses_complex_expression() {
    let input = WorkspaceInputLoader::new(fixture_dir()).load().expect("load fixtures");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
    let result = run_dl_query(
        ReasonerId::Dl,
        &input,
        "ex:hasRecord some ex:MedicalRecord",
        &namespaces,
        DlQueryMode::Inferred,
    )
    .expect("complex dl query");
    assert!(!result.normalized.is_empty());
    assert!(!result.query_class_iri.is_empty());
}

#[test]
fn dl_query_asserted_mode_returns_named_class_instances() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("abox.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:Person a owl:Class .\n",
            "ex:Employee a owl:Class ; rdfs:subClassOf ex:Person .\n",
            "ex:alice a owl:NamedIndividual , ex:Person .\n",
            "ex:bob a owl:NamedIndividual , ex:Employee .\n",
            "ex:carol a owl:NamedIndividual , owl:Thing .\n",
        ),
    )
    .unwrap();
    let input = WorkspaceInputLoader::new(ws).load().expect("load");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org#".to_string());
    let result =
        run_dl_query(ReasonerId::Rl, &input, "ex:Person", &namespaces, DlQueryMode::Asserted)
            .expect("asserted dl query");
    assert!(
        result.instances.iter().any(|i| i.ends_with("#alice")),
        "alice should be asserted instance: {:?}",
        result.instances
    );
    assert!(
        result.instances.iter().any(|i| i.ends_with("#bob")),
        "bob (Employee subclass) should be included: {:?}",
        result.instances
    );
    assert!(
        !result.instances.iter().any(|i| i.ends_with("#carol")),
        "carol is only Thing: {:?}",
        result.instances
    );
    assert!(
        !result.warnings.iter().any(|w| w.contains("instances require inferred")),
        "unexpected warning: {:?}",
        result.warnings
    );
}

#[test]
fn dl_query_asserted_mode_warns_for_anonymous_instances() {
    let input = WorkspaceInputLoader::new(fixture_dir()).load().expect("load fixtures");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org/clinic#".to_string());
    let result = run_dl_query(
        ReasonerId::Rl,
        &input,
        "ex:hasRecord some ex:MedicalRecord",
        &namespaces,
        DlQueryMode::Asserted,
    )
    .expect("anonymous asserted dl query");
    assert!(result.instances.is_empty());
    assert!(
        result.warnings.iter().any(|w| w.contains("anonymous")),
        "expected anonymous warning: {:?}",
        result.warnings
    );
}

#[test]
fn dl_query_asserted_equivalents_and_instances() {
    // #370 / #373 — asserted EquivalentClasses feed Equivalents + Instances.
    let tmp = tempfile::TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("equiv.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:Person a owl:Class .\n",
            "ex:Human a owl:Class ; owl:equivalentClass ex:Person .\n",
            "ex:alice a owl:NamedIndividual , ex:Human .\n",
        ),
    )
    .unwrap();
    let input = WorkspaceInputLoader::new(ws).load().expect("load");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org#".to_string());
    let result =
        run_dl_query(ReasonerId::Rl, &input, "ex:Person", &namespaces, DlQueryMode::Asserted)
            .expect("asserted equivalents dl query");
    assert!(
        result.equivalents.iter().any(|iri| iri.ends_with("#Human")),
        "Human should be equivalent: {:?}",
        result.equivalents
    );
    assert!(
        result.instances.iter().any(|i| i.ends_with("#alice")),
        "alice typed as Human should be instance of Person: {:?}",
        result.instances
    );
    assert!(
        !result.subclasses.iter().any(|iri| iri.ends_with("#Human")),
        "equivalents must not appear as subclasses: {:?}",
        result.subclasses
    );
    assert!(
        !result.superclasses.iter().any(|iri| iri.ends_with("#Human")),
        "equivalents must not appear as superclasses: {:?}",
        result.superclasses
    );
}

#[test]
fn dl_query_inferred_equivalents_excluded_from_sub_super() {
    // #370 / #372 — inferred mutual subsumption → Equivalents only.
    let tmp = tempfile::TempDir::new().unwrap();
    let ws = tmp.path();
    std::fs::write(
        ws.join("equiv.ttl"),
        concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
            "ex:A a owl:Class .\n",
            "ex:B a owl:Class ; owl:equivalentClass ex:A .\n",
        ),
    )
    .unwrap();
    let input = WorkspaceInputLoader::new(ws).load().expect("load");
    let mut namespaces = BTreeMap::new();
    namespaces.insert("ex".to_string(), "http://example.org#".to_string());
    let result = run_dl_query(ReasonerId::Dl, &input, "ex:A", &namespaces, DlQueryMode::Inferred)
        .expect("inferred equivalents dl query");
    assert!(
        result.equivalents.iter().any(|iri| iri.ends_with("#B")),
        "B should be equivalent: {:?}",
        result.equivalents
    );
    let equiv_set: std::collections::BTreeSet<_> = result.equivalents.iter().cloned().collect();
    assert!(
        result.subclasses.iter().all(|iri| !equiv_set.contains(iri)),
        "equivalents must not appear in subclasses: {:?}",
        result.subclasses
    );
    assert!(
        result.superclasses.iter().all(|iri| !equiv_set.contains(iri)),
        "equivalents must not appear in superclasses: {:?}",
        result.superclasses
    );
}
