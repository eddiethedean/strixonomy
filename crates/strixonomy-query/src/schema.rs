//! SQL virtual table schema metadata for the query workbench.

use serde::Serialize;
use strixonomy_catalog::OntologyCatalog;

#[derive(Debug, Clone, Serialize)]
pub struct SqlColumnSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub column_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SqlTableSchema {
    pub name: String,
    pub columns: Vec<SqlColumnSchema>,
}

/// Static table definitions (columns are stable for v0.13).
pub fn list_sql_tables() -> Vec<SqlTableSchema> {
    vec![
        table(
            "ontologies",
            &["id", "path", "format", "base_iri", "parse_status", "content_hash", "modified_time"],
        ),
        table(
            "classes",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "object_properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "data_properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "annotation_properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "individuals",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "entities",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table("annotations", &["subject", "predicate", "object", "ontology_id"]),
        table("axioms", &["id", "ontology_id", "subject", "predicate", "object", "axiom_kind"]),
        table("restrictions", &["class_iri", "property_iri", "restriction_kind", "filler"]),
        table("equivalent_class_axioms", &["class_iri", "expression"]),
        table("disjoint_class_axioms", &["class_iri", "disjoint_with"]),
        table("domain_axioms", &["property_iri", "domain"]),
        table("range_axioms", &["property_iri", "range"]),
        table("namespaces", &["prefix", "iri", "ontology_id"]),
        table("imports", &["ontology_id", "import_iri"]),
        table(
            "diagnostics",
            &["code", "severity", "message", "file", "line", "column", "entity_iri"],
        ),
    ]
}

fn table(name: &str, columns: &[&str]) -> SqlTableSchema {
    SqlTableSchema {
        name: name.to_string(),
        columns: columns
            .iter()
            .map(|c| SqlColumnSchema { name: (*c).to_string(), column_type: "string".to_string() })
            .collect(),
    }
}

/// Returns schema; `catalog` reserved for future dynamic columns.
pub fn list_sql_schema(_catalog: &OntologyCatalog) -> Vec<SqlTableSchema> {
    list_sql_tables()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_axiom_tables() {
        let names: Vec<_> = list_sql_tables().into_iter().map(|t| t.name).collect();
        for table in [
            "restrictions",
            "equivalent_class_axioms",
            "disjoint_class_axioms",
            "domain_axioms",
            "range_axioms",
        ] {
            assert!(names.contains(&table.to_string()), "missing table {table}");
        }
    }

    #[test]
    fn restrictions_table_has_expected_columns() {
        let table = list_sql_tables()
            .into_iter()
            .find(|t| t.name == "restrictions")
            .expect("restrictions table");
        let cols: Vec<_> = table.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(cols, vec!["class_iri", "property_iri", "restriction_kind", "filler"]);
    }
}
