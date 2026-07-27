//! Index a workspace via the Strixonomy `Workspace` API.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p strixonomy-workspace --example strixonomy_workspace
//! ```

use strixonomy::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = std::path::Path::new("fixtures");
    let ws = Workspace::open(workspace)?;

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

    let hits = ws.search("Person");
    println!("\nSearch 'Person': {} hit(s)", hits.len());
    for hit in hits.iter().take(5) {
        println!("  {} ({})", hit.entity.short_name, hit.entity.iri);
    }

    Ok(())
}
