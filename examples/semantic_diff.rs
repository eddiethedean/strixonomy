//! Compare ontology workspaces or git refs with semantic diff.

use strixonomy::diff::{diff_git_refs, format_diff_text, parse_git_range};
use strixonomy::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let ws = Workspace::open(&fixtures)?;
    println!("Indexed {} classes in fixtures/", ws.stats().class_count);

    if let Ok((left, right)) = parse_git_range("HEAD..WORKTREE") {
        if let Ok(diff) = diff_git_refs(&fixtures, &left, &right) {
            if diff.is_empty() {
                println!("\nNo uncommitted changes under fixtures/ (expected in a clean clone).");
                println!("Tip: edit a .ttl file and re-run, or use Workspace::diff_against_path in your own project.");
            } else {
                println!("\n{}", format_diff_text(&diff, false));
            }
            return Ok(());
        }
    }

    println!("\nGit worktree diff unavailable — showing self-diff demo (always empty).");
    let other = Workspace::open(&fixtures)?;
    let diff = ws.diff(&other);
    let counts = diff.summary_counts();
    println!(
        "Self-diff summary: {} entity, {} axiom, {} annotation changes",
        counts.entities, counts.axioms, counts.annotations
    );
    Ok(())
}
