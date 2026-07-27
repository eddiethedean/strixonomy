//! Protégé Wave 3 port: OBO Foundry registry JSON model/parser.
//! Upstream: OboFoundryContact/Entry/License/Registry/RegistryParser.

use std::path::PathBuf;
use strixonomy_obo::{parse_registry_json, OboFoundryRegistry};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/protege-roundtrip/ported/obofoundry_minimal.json")
}

#[test]
fn obofoundry_parse_minimal_fixture() {
    let bytes = std::fs::read(fixture_path()).expect("read fixture");
    let reg = parse_registry_json(&bytes).expect("parse");
    assert_eq!(reg.len(), 3);
    assert!(!reg.is_empty());
}

#[test]
fn obofoundry_lookup_bfo_and_go() {
    let bytes = std::fs::read(fixture_path()).expect("read");
    let reg = parse_registry_json(&bytes).expect("parse");

    let bfo = reg.get("bfo").expect("bfo");
    assert_eq!(bfo.title.as_deref(), Some("Basic Formal Ontology"));
    assert_eq!(bfo.activity_status.as_deref(), Some("active"));
    assert!(bfo.ontology_purl.as_deref().unwrap_or("").contains("bfo.owl"));
    assert_eq!(bfo.contact.as_ref().and_then(|c| c.label.as_deref()), Some("Barry Smith"));
    assert_eq!(bfo.license.as_ref().and_then(|l| l.label.as_deref()), Some("CC-BY"));

    let go = reg.get("go").expect("go");
    assert_eq!(go.title.as_deref(), Some("Gene Ontology"));
}

#[test]
fn obofoundry_missing_id_and_empty() {
    let bytes = std::fs::read(fixture_path()).expect("read");
    let reg = parse_registry_json(&bytes).expect("parse");
    assert!(reg.get("no-such-ontology").is_none());

    let empty = OboFoundryRegistry::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert!(empty.get("bfo").is_none());
}

#[test]
fn obofoundry_obsolete_entry_present() {
    let bytes = std::fs::read(fixture_path()).expect("read");
    let reg = parse_registry_json(&bytes).expect("parse");
    let obs = reg.get("obsolete-example").expect("obsolete");
    assert_eq!(obs.activity_status.as_deref(), Some("obsolete"));
    assert!(obs.contact.is_none());
}

#[test]
fn obofoundry_expanded_fixture_stress() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/protege-roundtrip/ported/obofoundry_expanded.json");
    let bytes = std::fs::read(&path).expect("read expanded");
    let reg = parse_registry_json(&bytes).expect("parse expanded");
    assert!(reg.len() >= 20, "expected tens of entries, got {}", reg.len());
    assert!(reg.get("uberon").is_some());
    assert!(reg.get("mondo").is_some());
    let sparse = reg.get("sparse").expect("sparse entry");
    assert!(sparse.title.is_none());
    assert_eq!(sparse.activity_status.as_deref(), Some("active"));
}
