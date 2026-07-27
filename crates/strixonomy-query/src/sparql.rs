use crate::sparql_update::is_sparql_update;
use crate::QueryError;
use oxigraph::sparql::{QueryResults, QuerySolution};
use serde::Serialize;
use std::collections::BTreeMap;
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::limits::{MAX_QUERY_BYTES, MAX_SPARQL_RESULT_ROWS};

pub type Result<T> = std::result::Result<T, QueryError>;

#[derive(Debug, Clone, Serialize)]
pub struct SparqlResult {
    pub columns: Vec<String>,
    pub rows: Vec<BTreeMap<String, String>>,
    pub truncated: bool,
}

pub fn run_sparql(catalog: &OntologyCatalog, sparql: &str) -> Result<SparqlResult> {
    if sparql.len() > MAX_QUERY_BYTES {
        return Err(QueryError::Sparql(format!(
            "query exceeds maximum length of {MAX_QUERY_BYTES} bytes"
        )));
    }

    if is_sparql_update(sparql) {
        return Err(QueryError::Sparql(
            "SPARQL update operations are not supported; use read-only queries".to_string(),
        ));
    }

    let results = catalog.store().query(sparql).map_err(|e| QueryError::Sparql(e.to_string()))?;

    match results {
        QueryResults::Solutions(solutions) => {
            let columns: Vec<String> =
                solutions.variables().iter().map(|v| v.as_str().to_string()).collect();
            let mut rows = Vec::new();
            let mut truncated = false;
            for solution in solutions {
                if rows.len() >= MAX_SPARQL_RESULT_ROWS {
                    truncated = true;
                    break;
                }
                let solution = solution.map_err(|e| QueryError::Sparql(e.to_string()))?;
                rows.push(solution_to_row(&solution, &columns));
            }
            Ok(SparqlResult { columns, rows, truncated })
        }
        QueryResults::Boolean(value) => {
            let mut row = BTreeMap::new();
            row.insert("boolean".into(), value.to_string());
            Ok(SparqlResult { columns: vec!["boolean".into()], rows: vec![row], truncated: false })
        }
        QueryResults::Graph(_) => Err(QueryError::Sparql(
            "CONSTRUCT/DESCRIBE graph results are not supported in v0.1".to_string(),
        )),
    }
}

fn solution_to_row(solution: &QuerySolution, columns: &[String]) -> BTreeMap<String, String> {
    let mut row = BTreeMap::new();
    for col in columns {
        let value = solution.get(col.as_str()).map(|term| term.to_string()).unwrap_or_default();
        row.insert(col.clone(), value);
    }
    row
}

pub fn to_json(result: &SparqlResult) -> Result<String> {
    serde_json::to_string_pretty(result).map_err(|e| QueryError::Export(e.to_string()))
}
