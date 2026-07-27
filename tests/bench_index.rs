//! Index + query smoke benchmarks (CI-friendly, no external downloads required).

use std::path::PathBuf;
use std::time::Instant;

use strixonomy_catalog::IndexBuilder;
use strixonomy_query::query_catalog;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[test]
fn bench_index_smoke() {
    let start = Instant::now();
    let catalog = IndexBuilder::new().workspace(fixtures_dir()).build().expect("index fixtures");
    let index_ms = start.elapsed().as_millis();
    eprintln!("bench_index_smoke: index fixtures in {index_ms}ms");
    assert!(index_ms < 30_000, "fixture index should finish within 30s (got {index_ms}ms)");

    let query_start = Instant::now();
    let result = query_catalog(&catalog, "SELECT short_name FROM classes").expect("query classes");
    let query_ms = query_start.elapsed().as_millis();
    eprintln!("bench_index_smoke: query classes -> {} rows in {query_ms}ms", result.rows.len());
    assert!(
        result.rows.iter().any(|r| r.get("short_name").map(|s| s == "Person").unwrap_or(false)),
        "expected Person class in fixtures: {:?}",
        result.rows
    );
    assert!(query_ms < 5_000);
}

#[test]
fn bench_axiom_tables_smoke() {
    let catalog = IndexBuilder::new().workspace(fixtures_dir()).build().expect("index");
    let domain = query_catalog(&catalog, "SELECT * FROM domain_axioms").expect("domain_axioms");
    assert!(!domain.columns.is_empty(), "domain_axioms must expose a schema");
    assert!(
        domain.rows.iter().any(|r| {
            r.values()
                .any(|v| v.contains("hasRecord") || v.contains("worksFor") || v.contains("age"))
        }),
        "expected a known property in domain_axioms: {:?}",
        domain.rows
    );

    let range = query_catalog(&catalog, "SELECT * FROM range_axioms").expect("range_axioms");
    assert!(!range.columns.is_empty());

    let disjoint =
        query_catalog(&catalog, "SELECT * FROM disjoint_class_axioms").expect("disjoint");
    assert!(
        disjoint.rows.iter().any(|r| r.values().any(|v| v.contains("Cat") || v.contains("Dog"))),
        "expected Cat/Dog disjoint row: {:?}",
        disjoint.rows
    );

    for table in ["restrictions", "equivalent_class_axioms"] {
        let sql = format!("SELECT * FROM {table}");
        let result = query_catalog(&catalog, &sql).expect(&sql);
        assert!(!result.columns.is_empty(), "{table} must expose columns even if empty");
        eprintln!("bench_axiom_tables_smoke: {table} -> {} rows", result.rows.len());
    }
}
