use std::path::PathBuf;
use strixonomy_reasoner::{
    check_consistency, check_instance, classify, realize, ReasonerId, WorkspaceInputLoader,
};

fn load(rel: &str) -> strixonomy_reasoner::ReasonerInput {
    let dir = tempfile::tempdir().expect("tempdir");
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    let name = src.file_name().unwrap();
    std::fs::copy(&src, dir.path().join(name)).expect("copy");
    WorkspaceInputLoader::new(dir.path()).load().expect("load")
}

#[test]
fn realize_employee_alice() {
    let input = load("tests/fixtures/reasoner/abox/realize.ttl");
    let result = realize(ReasonerId::Rl, &input).expect("realize");
    let alice = result
        .individuals
        .iter()
        .find(|e| e.individual_iri.ends_with("#alice") || e.individual_iri.contains("alice"))
        .expect("alice");
    assert!(
        alice.types.iter().any(|t| t.contains("Employee") || t.contains("Person")),
        "alice types: {:?}",
        alice.types
    );
}

#[test]
fn check_instance_alice_employee() {
    let input = load("tests/fixtures/reasoner/abox/realize.ttl");
    let alice = "http://example.org/abox#alice";
    let employee = "http://example.org/abox#Employee";
    let result = check_instance(ReasonerId::Rl, &input, alice, employee).expect("check");
    assert!(result.entailed, "alice should be Employee");
}

#[test]
fn classify_reports_abox_clash() {
    let input = load("tests/fixtures/reasoner/abox/sameas_clash.ttl");
    let result = classify(ReasonerId::Rl, &input, false).expect("classify");
    let consistency = check_consistency(ReasonerId::Rl, &input).expect("consistency");
    assert!(
        !consistency.consistent
            || result.warnings.iter().any(|w| w.code == "abox_clash")
            || consistency
                .detail
                .as_ref()
                .is_some_and(|d| !d.abox_clashes.is_empty() || !d.ontology_consistent),
        "expected ABox inconsistency: {:?}",
        consistency
    );
}

#[test]
fn realize_reports_not_truncated_for_small_fixture() {
    let input = load("tests/fixtures/reasoner/abox/realize.ttl");
    let result = realize(ReasonerId::Rl, &input).expect("realize");
    assert!(
        !result.truncated,
        "small fixture should fully realize: individuals={}",
        result.individuals.len()
    );
    assert_eq!(result.entailment_errors, 0);
}
