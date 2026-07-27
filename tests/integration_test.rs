mod golden;
mod support;

use std::path::PathBuf;

use strixonomy_catalog::IndexBuilder;
use strixonomy_core::ParseStatus;
use strixonomy_query::{query_catalog, sparql_catalog, QueryError};
use support::fixture_catalog;

#[test]
fn indexes_fixture_ontology() {
    let stats = fixture_catalog().data().stats();

    assert_eq!(stats.ontology_count, 6);
    assert_eq!(stats.class_count, 18);
    assert_eq!(stats.object_property_count, 5);
    assert_eq!(stats.data_property_count, 1);
    assert_eq!(stats.annotation_property_count, 1);
    assert_eq!(stats.individual_count, 2);
    assert_eq!(stats.error_count, 0);
    assert!(stats.axiom_count >= 1);
    assert!(stats.annotation_count > 0);
    assert!(stats.triple_count > 0);
}

#[test]
fn validate_fails_on_parse_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("bad.ttl"), "@prefix ex: <http://ex/> .\nex:a a owl:Class .\n")
        .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
    assert_eq!(catalog.data().stats().error_count, 1);

    let doc = catalog.data().documents.iter().find(|d| d.parse_status == ParseStatus::Error);
    assert!(doc.is_some(), "expected parse error document");
    assert!(doc.unwrap().parse_message.is_some());
}

#[test]
fn valid_file_still_indexes_when_sibling_has_parse_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(support::fixture_workspace().join("example.ttl"), dir.path().join("good.ttl"))
        .unwrap();
    std::fs::write(dir.path().join("bad.ttl"), "@prefix ex: <http://ex/> .\nex:a a owl:Class .\n")
        .unwrap();

    let stats = IndexBuilder::new().workspace(dir.path()).build().expect("build").data().stats();

    assert_eq!(stats.ontology_count, 2);
    assert_eq!(stats.error_count, 1);
    assert_eq!(stats.class_count, 3);
}

#[test]
fn sql_diagnostics_table() {
    let dir = tempfile::tempdir().unwrap();
    for name in ["lint-broken-import.ttl", "lint-duplicate-labels.ttl", "lint-orphan.ttl"] {
        std::fs::copy(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/diagnostics").join(name),
            dir.path().join(name),
        )
        .unwrap();
    }

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
    let result = query_catalog(&catalog, "SELECT code, severity FROM diagnostics")
        .expect("diagnostics query");

    let rows: Vec<_> = result
        .rows
        .iter()
        .map(|r| {
            (
                r.get("code").cloned().unwrap_or_default(),
                r.get("severity").cloned().unwrap_or_default(),
            )
        })
        .collect();

    assert!(rows.iter().any(|(c, s)| c == "broken_import" && s == "error"));
    assert!(rows.iter().any(|(c, s)| c == "duplicate_label" && s == "warning"));
    assert!(rows.iter().any(|(c, s)| c == "orphan_class" && s == "warning"));
    assert!(rows.iter().any(|(c, s)| c == "missing_label" && s == "warning"));
    assert!(rows.iter().filter(|(c, _)| c == "duplicate_label").count() >= 2);
}

#[test]
fn validate_reports_diagnostics() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/diagnostics/lint-broken-import.ttl"),
        dir.path().join("bad.ttl"),
    )
    .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
    let errors = catalog
        .data()
        .diagnostics
        .iter()
        .filter(|d| d.severity == strixonomy_core::DiagnosticSeverity::Error)
        .count();
    assert!(errors >= 1);
}

#[test]
fn fixture_diagnostics_use_real_file_paths() {
    let catalog = fixture_catalog();
    let diags = &catalog.data().diagnostics;
    for diag in diags {
        assert!(
            diag.file.file_name().is_some(),
            "diagnostic should not use '.' fallback: {:?}",
            diag
        );
        assert_ne!(diag.file, std::path::Path::new("."), "bad file path for {:?}", diag);
    }
}

#[test]
fn fixture_rdf_xml_has_no_spurious_undefined_prefix() {
    let catalog = fixture_catalog();
    let owl_path = support::fixture_workspace().join("organization.owl");
    let undefined_on_owl: Vec<_> = catalog
        .data()
        .diagnostics
        .iter()
        .filter(|d| {
            d.code == strixonomy_core::DiagnosticCode::UndefinedPrefix && d.file == owl_path
        })
        .collect();
    assert!(
        undefined_on_owl.is_empty(),
        "unexpected undefined_prefix on organization.owl: {:?}",
        undefined_on_owl
    );
}

#[test]
fn fixture_root_class_not_orphan() {
    let catalog = fixture_catalog();
    let thing_orphan = catalog.data().diagnostics.iter().any(|d| {
        d.code == strixonomy_core::DiagnosticCode::OrphanClass
            && d.entity_iri.as_deref() == Some("http://example.org/people#Thing")
    });
    assert!(!thing_orphan, "ex:Thing should not be flagged orphan");
}

#[test]
fn open_buffer_diagnostics_detect_undefined_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("live.ttl");
    let base = "@prefix ex: <http://example.org/live#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<http://example.org/live> a owl:Ontology .\n";
    std::fs::write(&path, format!("{base}ex:Ok a owl:Class .\n")).unwrap();

    let mut overrides = std::collections::HashMap::new();
    overrides.insert(path.clone(), format!("{base}ex:Ok a owl:Class .\nun:Bad a owl:Class .\n"));

    let catalog = IndexBuilder::new()
        .workspace(dir.path())
        .document_overrides(overrides)
        .build()
        .expect("build");

    let diags: Vec<_> = catalog
        .data()
        .diagnostics
        .iter()
        .filter(|d| d.file.file_name() == path.file_name())
        .collect();

    let undef = diags
        .iter()
        .find(|d| {
            d.message.contains("un:")
                && (d.code == strixonomy_core::DiagnosticCode::UndefinedPrefix
                    || d.code == strixonomy_core::DiagnosticCode::ParseError)
        })
        .unwrap_or_else(|| panic!("expected undeclared prefix diagnostic, got: {diags:?}"));
    assert_eq!(undef.severity, strixonomy_core::DiagnosticSeverity::Error);
}

#[test]
fn classes_snapshot() {
    golden::assert_golden_snapshot(
        "fixtures",
        "SELECT short_name, labels FROM classes",
        &PathBuf::from("tests/golden/snapshots/classes.tsv"),
    );
}

#[test]
fn sql_query_with_filter() {
    let catalog = fixture_catalog();

    let result =
        query_catalog(&catalog, "SELECT short_name FROM classes WHERE short_name = 'Person'")
            .expect("filtered query");

    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0].get("short_name").map(String::as_str), Some("Person"));
}

#[test]
fn sql_query_properties_and_axioms_tables() {
    let catalog = fixture_catalog();

    let properties =
        query_catalog(&catalog, "SELECT short_name, kind FROM properties").expect("properties");
    assert_eq!(properties.rows.len(), 7);

    let axioms = query_catalog(&catalog, "SELECT axiom_kind FROM axioms").expect("axioms");
    assert!(axioms.rows.iter().any(|row| row.get("axiom_kind").map(String::as_str)
        == Some(strixonomy_core::AXIOM_KIND_SUB_CLASS_OF)));

    let imports = query_catalog(&catalog, "SELECT import_iri FROM imports").expect("imports");
    assert!(!imports.rows.is_empty());
}

#[test]
fn sql_query_unknown_table_returns_error() {
    let catalog = fixture_catalog();

    let err = query_catalog(&catalog, "SELECT * FROM missing_table").unwrap_err();
    assert!(matches!(err, QueryError::Sql(msg) if msg.contains("unknown table")));
}

#[test]
fn sql_query_non_select_returns_error() {
    let catalog = fixture_catalog();

    let err = query_catalog(&catalog, "INSERT INTO classes VALUES ('x')").unwrap_err();
    assert!(matches!(err, QueryError::Sql(msg) if msg.contains("only SELECT")));
}

#[test]
fn sparql_query_triples() {
    let catalog = fixture_catalog();

    let result = sparql_catalog(
        &catalog,
        "SELECT ?s WHERE { ?s a <http://www.w3.org/2002/07/owl#NamedIndividual> }",
    )
    .expect("sparql query");

    assert_eq!(result.rows.len(), 2);
    let subjects: Vec<_> = result.rows.iter().filter_map(|row| row.get("s").cloned()).collect();
    assert!(subjects.iter().any(|s| s.contains("alice")));
    assert!(subjects.iter().any(|s| s.contains("acme")));
}

#[test]
fn sparql_ask_query_returns_boolean() {
    let catalog = fixture_catalog();

    let result = sparql_catalog(
        &catalog,
        "ASK { <http://example.org/people#Person> a <http://www.w3.org/2002/07/owl#Class> }",
    )
    .expect("sparql ask");

    assert_eq!(result.columns, vec!["boolean"]);
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0].get("boolean").map(String::as_str), Some("true"));
}

#[test]
fn sparql_malformed_query_returns_error() {
    let catalog = fixture_catalog();

    let err = sparql_catalog(&catalog, "SELECT ?s WHERE { ?s ?p ?o").unwrap_err();
    assert!(matches!(err, QueryError::Sparql(_)));
}

#[test]
fn sparql_multi_file_blank_nodes_do_not_collide() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("a.ttl"),
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ex: <http://ex.org/a#> .
ex:A a owl:Class ;
  rdfs:subClassOf [ a owl:Restriction ; owl:onProperty ex:p1 ; owl:someValuesFrom ex:B ] .
ex:B a owl:Class .
ex:p1 a owl:ObjectProperty .
"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("b.ttl"),
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ex: <http://ex.org/b#> .
ex:X a owl:Class ;
  rdfs:subClassOf [ a owl:Restriction ; owl:onProperty ex:p2 ; owl:someValuesFrom ex:Y ] .
ex:Y a owl:Class .
ex:p2 a owl:ObjectProperty .
"#,
    )
    .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");

    let fused = sparql_catalog(
        &catalog,
        r#"ASK {
  ?r <http://www.w3.org/2002/07/owl#onProperty> <http://ex.org/a#p1> .
  ?r <http://www.w3.org/2002/07/owl#someValuesFrom> <http://ex.org/b#Y> .
}"#,
    )
    .expect("sparql ask fused");
    assert_eq!(
        fused.rows[0].get("boolean").map(String::as_str),
        Some("false"),
        "restrictions from different files must not share blank nodes"
    );

    let intact = sparql_catalog(
        &catalog,
        r#"ASK {
  <http://ex.org/a#A> <http://www.w3.org/2000/01/rdf-schema#subClassOf> ?r .
  ?r <http://www.w3.org/2002/07/owl#onProperty> <http://ex.org/a#p1> .
  ?r <http://www.w3.org/2002/07/owl#someValuesFrom> <http://ex.org/a#B> .
}"#,
    )
    .expect("sparql ask intact");
    assert_eq!(intact.rows[0].get("boolean").map(String::as_str), Some("true"));
}

#[test]
fn sql_json_and_csv_export() {
    let catalog = fixture_catalog();
    let result = query_catalog(&catalog, "SELECT short_name FROM classes").expect("query");

    let json = strixonomy_query::sql::to_json(&result).expect("json export");
    assert!(json.contains("Person"));

    let csv = strixonomy_query::sql::to_csv(&result).expect("csv export");
    assert!(csv.contains("short_name"));
    assert!(csv.contains("Person"));
}
