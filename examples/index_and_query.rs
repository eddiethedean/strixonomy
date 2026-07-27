//! Index a workspace and run SQL-like queries over the ontology catalog.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p strixonomy-workspace --example index_and_query
//! ```

use strixonomy::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let ws = Workspace::open(&workspace)?;

    let stats = ws.stats();
    println!(
        "Indexed {} ontologies, {} classes, {} triples",
        stats.ontology_count, stats.class_count, stats.triple_count
    );

    let result = ws.query("SELECT short_name, labels FROM classes")?;
    for row in &result.rows {
        println!(
            "{} — {}",
            row.get("short_name").map(String::as_str).unwrap_or(""),
            row.get("labels").map(String::as_str).unwrap_or("")
        );
    }

    Ok(())
}
