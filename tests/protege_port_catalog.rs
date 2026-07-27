//! Protégé Wave 4: catalog-v001.xml / XmlBase redirect oracles.
//! Upstream: Folder_IT, XmlBaseTest (vendored fixture themes, not OSGi).

mod support;

use strixonomy_catalog::{load_xml_catalog, parse_xml_catalog};
use strixonomy_refactor::resolve_import_document;
use support::protege_port::{copy_ported_tree, index_workspace, ported_dir};

#[test]
fn catalog_parse_fixture_uri_redirect() {
    let path = ported_dir().join("catalog_home/catalog-v001.xml");
    let cat = load_xml_catalog(&path).expect("load catalog");
    assert_eq!(
        cat.resolve_uri("http://example.org/imported/remote-lib").as_deref(),
        Some("lib.ttl")
    );
}

#[test]
fn catalog_xml_base_relative_uri() {
    let xml = r#"<?xml version="1.0"?>
<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <group xml:base="nested/">
    <uri name="http://example.org/x" uri="x.ttl"/>
  </group>
</catalog>"#;
    let path = std::path::Path::new("/tmp/catalog-v001.xml");
    let cat = parse_xml_catalog(path, xml).unwrap();
    assert_eq!(cat.resolve_uri("http://example.org/x").as_deref(), Some("nested/x.ttl"));
}

#[test]
fn catalog_resolves_import_when_iri_differs_from_ontology_id() {
    let dir = copy_ported_tree("catalog_home");
    let catalog = index_workspace(dir.path());
    let remote = "http://example.org/imported/remote-lib";
    // Without redirect, the lib document's base_iri is http://example.org/lib
    assert!(
        catalog.data().documents.iter().any(|d| d.path.ends_with("lib.ttl")),
        "lib.ttl should be indexed"
    );
    let resolved = resolve_import_document(&catalog, remote)
        .expect("catalog-v001 should redirect remote-lib → lib.ttl");
    assert!(
        resolved.path.ends_with("lib.ttl"),
        "expected lib.ttl, got {}",
        resolved.path.display()
    );
}
