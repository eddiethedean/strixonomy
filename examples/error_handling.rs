//! Match structured errors from the Strixonomy façade and underlying crates.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p strixonomy-workspace --example error_handling
//! ```

use std::path::Path;
use strixonomy::{Error, Workspace};
use strixonomy_parser::{parse_ontology_file, ParseError};

fn main() {
    let missing = Path::new("fixtures/does-not-exist.ttl");
    if let Err(err) =
        parse_ontology_file(missing, strixonomy_core::OntologyFormat::Turtle, "doc-1", "h", 0)
    {
        match err {
            ParseError::Io(_) => eprintln!("parse: file not found or unreadable"),
            ParseError::LimitExceeded(msg) => eprintln!("parse: {msg}"),
            other => eprintln!("parse: {other}"),
        }
    }

    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    if let Err(err) = workspace_query_unknown_table(&fixtures) {
        match err {
            Error::Catalog(catalog) => eprintln!("catalog: {catalog}"),
            Error::Query(query) => eprintln!("query: {query}"),
            other => eprintln!("strixonomy: {other}"),
        }
    }
}

fn workspace_query_unknown_table(fixtures: &Path) -> Result<(), Error> {
    let ws = Workspace::open(fixtures)?;
    ws.query("SELECT * FROM not_a_table")?;
    Ok(())
}
