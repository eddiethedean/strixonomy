//! Integration tests for v0.13 Horned-OWL SQL axiom virtual tables.

mod support;

use std::path::PathBuf;

use strixonomy_catalog::IndexBuilder;
use strixonomy_query::query_catalog;

fn complex_classes_workspace() -> PathBuf {
    support::fixture_workspace()
}

#[test]
fn domain_and_range_axiom_tables_project_fixture_properties() {
    let catalog =
        IndexBuilder::new().workspace(complex_classes_workspace()).build().expect("index fixtures");

    let domain = query_catalog(&catalog, "SELECT property_iri, domain FROM domain_axioms")
        .expect("domain_axioms query");
    assert!(
        domain
            .rows
            .iter()
            .any(|r| { r.get("property_iri").map(|v| v.contains("hasRecord")).unwrap_or(false) }),
        "expected hasRecord domain row, got: {:?}",
        domain.rows
    );

    let range = query_catalog(&catalog, "SELECT property_iri, range FROM range_axioms")
        .expect("range_axioms query");
    assert!(
        range
            .rows
            .iter()
            .any(|r| { r.get("property_iri").map(|v| v.contains("hasRecord")).unwrap_or(false) }),
        "expected hasRecord range row, got: {:?}",
        range.rows
    );
}

#[test]
fn equivalent_class_axioms_include_staff_employee() {
    let catalog =
        IndexBuilder::new().workspace(complex_classes_workspace()).build().expect("index fixtures");

    let result =
        query_catalog(&catalog, "SELECT class_iri, expression FROM equivalent_class_axioms")
            .expect("equivalent_class_axioms query");

    assert!(
        result
            .rows
            .iter()
            .any(|r| { r.get("class_iri").map(|v| v.contains("Staff")).unwrap_or(false) }),
        "expected Staff equivalent class row, got: {:?}",
        result.rows
    );
}

#[test]
fn restrictions_table_projects_subclass_restriction_axioms() {
    let catalog =
        IndexBuilder::new().workspace(complex_classes_workspace()).build().expect("index fixtures");

    let result = query_catalog(
        &catalog,
        "SELECT class_iri, property_iri, restriction_kind FROM restrictions",
    )
    .expect("restrictions query");

    assert!(
        result.rows.iter().any(|r| {
            r.get("class_iri").map(|v| v.contains("Patient")).unwrap_or(false)
                && r.get("property_iri").map(|v| v.contains("hasRecord")).unwrap_or(false)
        }),
        "expected Patient restriction on hasRecord, got: {:?}",
        result.rows
    );
}

#[test]
fn list_sql_schema_matches_queryable_axiom_tables() {
    let catalog =
        IndexBuilder::new().workspace(complex_classes_workspace()).build().expect("index fixtures");

    let schema = strixonomy_query::list_sql_schema(&catalog);
    let names: Vec<_> = schema.iter().map(|t| t.name.as_str()).collect();
    for table in [
        "restrictions",
        "equivalent_class_axioms",
        "disjoint_class_axioms",
        "domain_axioms",
        "range_axioms",
    ] {
        assert!(names.contains(&table), "schema missing {table}");
        query_catalog(&catalog, &format!("SELECT * FROM {table}")).unwrap_or_else(|e| {
            panic!("query against schema table {table} failed: {e}");
        });
    }
}
