use crate::model::DiffResult;

pub fn format_diff_text(diff: &DiffResult, breaking_only: bool) -> String {
    let mut out = String::new();
    if !breaking_only {
        out.push_str(&format!(
            "Summary: {} entity, {} axiom, {} annotation, {} import, {} inference, {} breaking\n\n",
            diff.entity_changes.len(),
            diff.axiom_changes.len(),
            diff.annotation_changes.len(),
            diff.import_changes.len(),
            diff.inference_changes.len(),
            diff.breaking_changes.len(),
        ));
    }
    if !diff.breaking_changes.is_empty() {
        out.push_str("Breaking changes:\n");
        for b in &diff.breaking_changes {
            out.push_str(&format!("  - [{}] {}\n", reason_label(b.reason), b.message));
        }
        out.push('\n');
    }
    if breaking_only {
        return out;
    }
    if !diff.entity_changes.is_empty() {
        out.push_str("Entity changes:\n");
        for e in &diff.entity_changes {
            out.push_str(&format!("  - {:?} {}\n", e.kind, e.iri));
        }
        out.push('\n');
    }
    if !diff.axiom_changes.is_empty() {
        out.push_str("Axiom changes:\n");
        for a in &diff.axiom_changes {
            out.push_str(&format!(
                "  - {} {} {} {} {}\n",
                a.change, a.axiom_kind, a.subject, a.predicate, a.object
            ));
        }
        out.push('\n');
    }
    if !diff.annotation_changes.is_empty() {
        out.push_str("Annotation changes:\n");
        for a in &diff.annotation_changes {
            out.push_str(&format!("  - {} {} {} {}\n", a.change, a.subject, a.predicate, a.object));
        }
        out.push('\n');
    }
    if !diff.import_changes.is_empty() {
        out.push_str("Import changes:\n");
        for i in &diff.import_changes {
            out.push_str(&format!("  - {} {}\n", i.change, i.import_iri));
        }
        out.push('\n');
    }
    out
}

pub fn format_diff_markdown(diff: &DiffResult, breaking_only: bool) -> String {
    let mut out = String::from("# Ontology semantic diff\n\n");
    if !diff.breaking_changes.is_empty() {
        out.push_str("## Breaking changes\n\n");
        for b in &diff.breaking_changes {
            out.push_str(&format!("- **{}**: {}\n", reason_label(b.reason), b.message));
        }
        out.push('\n');
    }
    if breaking_only {
        return out;
    }
    out.push_str(&format!(
        "## Summary\n\n| Category | Count |\n|---|---|\n| Entities | {} |\n| Axioms | {} |\n| Annotations | {} |\n| Imports | {} |\n| Inferences | {} |\n| Breaking | {} |\n\n",
        diff.entity_changes.len(),
        diff.axiom_changes.len(),
        diff.annotation_changes.len(),
        diff.import_changes.len(),
        diff.inference_changes.len(),
        diff.breaking_changes.len(),
    ));
    if !diff.entity_changes.is_empty() {
        out.push_str("## Entity changes\n\n");
        for e in &diff.entity_changes {
            match &e.previous_iri {
                Some(prev) => {
                    out.push_str(&format!("- **{:?}** `{}` ← `{}`\n", e.kind, e.iri, prev))
                }
                None => out.push_str(&format!("- **{:?}** `{}`\n", e.kind, e.iri)),
            }
        }
        out.push('\n');
    }
    if !diff.axiom_changes.is_empty() {
        out.push_str("## Axiom changes\n\n");
        for a in &diff.axiom_changes {
            out.push_str(&format!(
                "- **{}** `{}` — {} {} {}\n",
                a.change, a.axiom_kind, a.subject, a.predicate, a.object
            ));
        }
        out.push('\n');
    }
    if !diff.annotation_changes.is_empty() {
        out.push_str("## Annotation changes\n\n");
        for a in &diff.annotation_changes {
            out.push_str(&format!(
                "- **{}** `{}` {} {}\n",
                a.change, a.subject, a.predicate, a.object
            ));
        }
        out.push('\n');
    }
    if !diff.import_changes.is_empty() {
        out.push_str("## Import changes\n\n");
        for i in &diff.import_changes {
            out.push_str(&format!("- **{}** {}\n", i.change, i.import_iri));
        }
        out.push('\n');
    }
    if !diff.inference_changes.is_empty() {
        out.push_str("## Inference changes\n\n");
        for i in &diff.inference_changes {
            out.push_str(&format!("- **{}** `{}` — {}\n", i.change, i.class_iri, i.detail));
        }
        out.push('\n');
    }
    out
}

pub fn format_diff_pr_summary(diff: &DiffResult) -> String {
    let mut out = String::from("## Ontology changes\n\n");
    if !diff.breaking_changes.is_empty() {
        out.push_str("> **Breaking changes detected** — review carefully before merge.\n\n");
        for b in &diff.breaking_changes {
            out.push_str(&format!("- **{}**: {}\n", reason_label(b.reason), b.message));
        }
        out.push('\n');
    }
    let counts = format!(
        "| Entities | Axioms | Annotations | Imports | Inferences |\n|---|---|---|---|---|\n| {} | {} | {} | {} | {} |\n\n",
        diff.entity_changes.len(),
        diff.axiom_changes.len(),
        diff.annotation_changes.len(),
        diff.import_changes.len(),
        diff.inference_changes.len(),
    );
    out.push_str(&counts);

    if !diff.entity_changes.is_empty() {
        out.push_str("### Entities\n\n");
        for e in diff.entity_changes.iter().take(50) {
            out.push_str(&format!("- `{:?}` {}\n", e.kind, e.iri));
        }
        if diff.entity_changes.len() > 50 {
            out.push_str(&format!(
                "\n_…and {} more entity changes_\n",
                diff.entity_changes.len() - 50
            ));
        }
        out.push('\n');
    }
    if !diff.axiom_changes.is_empty() {
        out.push_str("### Axioms\n\n");
        for a in diff.axiom_changes.iter().take(30) {
            out.push_str(&format!(
                "- **{}** `{}` — {} {} {}\n",
                a.change, a.axiom_kind, a.subject, a.predicate, a.object
            ));
        }
        if diff.axiom_changes.len() > 30 {
            out.push_str(&format!(
                "\n_…and {} more axiom changes_\n",
                diff.axiom_changes.len() - 30
            ));
        }
        out.push('\n');
    }
    if !diff.import_changes.is_empty() {
        out.push_str("### Imports\n\n");
        for i in &diff.import_changes {
            out.push_str(&format!("- **{}** {}\n", i.change, i.import_iri));
        }
        out.push('\n');
    }
    out.push_str("---\n_Generated by `strixonomy diff --pr-summary`_\n");
    out
}

pub fn format_diff_json(diff: &DiffResult) -> String {
    serde_json::to_string_pretty(diff).unwrap_or_else(|_| "{}".to_string())
}

fn reason_label(reason: crate::model::BreakingReason) -> &'static str {
    use crate::model::BreakingReason::*;
    match reason {
        RemovedEntity => "removed_entity",
        RenamedIri => "renamed_iri",
        RemovedSuperclass => "removed_superclass",
        RemovedImport => "removed_import",
        UnsatisfiableClass => "unsatisfiable_class",
        DomainRangeChange => "domain_range_change",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AxiomChange, BreakingChange, DiffResult, EntityChange, EntityChangeKind};

    #[test]
    fn pr_summary_includes_breaking_callout() {
        let diff = DiffResult {
            entity_changes: vec![EntityChange {
                iri: "http://ex#A".into(),
                kind: EntityChangeKind::Removed,
                previous_iri: None,
                labels: vec![],
            }],
            axiom_changes: vec![],
            annotation_changes: vec![],
            import_changes: vec![],
            inference_changes: vec![],
            breaking_changes: vec![BreakingChange {
                reason: crate::model::BreakingReason::RemovedEntity,
                message: "class removed".into(),
                entity_iri: Some("http://ex#A".into()),
            }],
        };
        let md = format_diff_pr_summary(&diff);
        assert!(md.contains("Breaking changes"));
        assert!(md.contains("http://ex#A"));
        assert!(md.contains("strixonomy diff --pr-summary"));
    }

    #[test]
    fn pr_summary_lists_axiom_rows() {
        let diff = DiffResult {
            entity_changes: vec![],
            axiom_changes: vec![AxiomChange {
                change: "added".into(),
                axiom_kind: "SubClassOf".into(),
                subject: "http://ex#Child".into(),
                predicate: "rdfs:subClassOf".into(),
                object: "http://ex#Parent".into(),
            }],
            annotation_changes: vec![],
            import_changes: vec![],
            inference_changes: vec![],
            breaking_changes: vec![],
        };
        let md = format_diff_pr_summary(&diff);
        assert!(md.contains("### Axioms"));
        assert!(md.contains("SubClassOf"));
    }

    #[test]
    fn markdown_includes_entity_and_axiom_sections() {
        let diff = DiffResult {
            entity_changes: vec![EntityChange {
                iri: "http://ex#A".into(),
                kind: EntityChangeKind::Added,
                previous_iri: None,
                labels: vec![],
            }],
            axiom_changes: vec![AxiomChange {
                change: "added".into(),
                axiom_kind: "SubClassOf".into(),
                subject: "http://ex#Child".into(),
                predicate: "rdfs:subClassOf".into(),
                object: "http://ex#Parent".into(),
            }],
            annotation_changes: vec![],
            import_changes: vec![],
            inference_changes: vec![],
            breaking_changes: vec![],
        };
        let md = format_diff_markdown(&diff, false);
        assert!(md.contains("## Entity changes"));
        assert!(md.contains("http://ex#A"));
        assert!(md.contains("## Axiom changes"));
        assert!(md.contains("SubClassOf"));
    }
}
