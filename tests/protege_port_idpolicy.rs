//! Protégé Wave 1 deepening: IdPolicyParser_* semantic oracles.
//! Upstream fixtures vendored under `examples/protege-roundtrip/ported/idpolicy/`.

mod support;

use strixonomy_obo::{parse_id_policy_file, parse_id_policy_from_catalog};
use support::protege_port::{copy_ported_tree, index_workspace, ported_dir};

#[test]
fn idpolicy_go_fixture_parses_policy_and_ranges() {
    let path = ported_dir().join("idpolicy/go-idranges.owl");
    let policy = parse_id_policy_file(&path).expect("GO idranges");
    assert_eq!(policy.ids_for, "GO");
    assert_eq!(policy.id_digits, 7);
    assert_eq!(policy.id_prefix, "http://purl.obolibrary.org/obo/GO_");
    assert_eq!(policy.ranges.len(), 24);
    let hill =
        policy.ranges.iter().find(|r| r.allocated_to == "David Hill").expect("David Hill range");
    assert_eq!(hill.min, 60001);
    assert_eq!(hill.max, 65000);
}

#[test]
fn idpolicy_empty_ranges_list() {
    let path = ported_dir().join("idpolicy/empty-idranges.owl");
    let policy = parse_id_policy_file(&path).expect("empty ranges");
    assert_eq!(policy.ids_for, "GO");
    assert_eq!(policy.id_digits, 7);
    assert_eq!(policy.id_prefix, "http://purl.obolibrary.org/obo/GO_");
    assert!(policy.ranges.is_empty());
}

#[test]
fn idpolicy_missing_id_prefix_fails() {
    let path = ported_dir().join("idpolicy/missing-idprefix-idranges.owl");
    let err = parse_id_policy_file(&path).unwrap_err();
    assert!(err.to_string().contains("Id prefix"), "expected missing prefix error, got {err}");
}

#[test]
fn idpolicy_missing_id_digit_count_fails() {
    let path = ported_dir().join("idpolicy/missing-iddigitcount-idranges.owl");
    let err = parse_id_policy_file(&path).unwrap_err();
    assert!(
        err.to_string().contains("digit count") || err.to_string().contains("Id digit"),
        "expected missing digit count error, got {err}"
    );
}

#[test]
fn idpolicy_missing_policy_for_fails() {
    let path = ported_dir().join("idpolicy/missing-policyfor-idranges.owl");
    let err = parse_id_policy_file(&path).unwrap_err();
    assert!(
        err.to_string().contains("Id policy for") || err.to_string().contains("policy for"),
        "expected missing idsfor error, got {err}"
    );
}

#[test]
fn idpolicy_from_catalog_documents() {
    let dir = copy_ported_tree("idpolicy");
    // Manchester OWL may not fully index; still register file paths via copy and parse by path suffix.
    let _catalog = index_workspace(dir.path());
    // Prefer direct file — catalog format may skip .owl Manchester; invent a synthetic OntologyDocument list.
    let path = dir.path().join("empty-idranges.owl");
    let docs = vec![strixonomy_core::OntologyDocument {
        id: "empty".into(),
        path: path.clone(),
        format: strixonomy_core::OntologyFormat::OwlXml, // placeholder; parse reads text
        base_iri: Some("http://purl.obolibrary.org/obo/go/go-idranges.owl".into()),
        version_iri: None,
        imports: vec![],
        namespaces: Default::default(),
        parse_status: strixonomy_core::ParseStatus::Ok,
        content_hash: String::new(),
        modified_time: 0,
        parse_message: None,
        parse_error_location: None,
    }];
    let policy = parse_id_policy_from_catalog(&docs, "empty").expect("from catalog");
    assert_eq!(policy.ids_for, "GO");
    assert!(policy.ranges.is_empty());
}
