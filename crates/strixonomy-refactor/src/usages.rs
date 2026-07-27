use crate::model::{Usage, UsageKind};
use crate::source::read_source_text;
use crate::text::is_token_match_at;
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::{
    document_matches_ontology_id, OntologyDocument, OntologyFormat, ParseStatus,
};
use strixonomy_owl::{namespaces_for_text, short_name_from_iri};

/// Find all usages of `target_iri` across the indexed workspace.
pub fn find_usages(catalog: &OntologyCatalog, target_iri: &str) -> Vec<Usage> {
    find_usages_with_overrides(catalog, target_iri, &HashMap::new())
}

/// Find usages, preferring unsaved document text from `document_overrides`.
pub fn find_usages_with_overrides(
    catalog: &OntologyCatalog,
    target_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Vec<Usage> {
    let mut usages = Vec::new();
    let data = catalog.data();
    let mut seen = BTreeSet::new();

    for entity in &data.entities {
        if entity.iri == target_iri {
            if let Some(doc) = catalog.entity_document(&entity.iri) {
                let key = (doc.path.clone(), UsageKind::EntityDeclaration, entity.iri.clone());
                if seen.insert(key.clone()) {
                    usages.push(Usage {
                        iri: entity.iri.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: entity.source_location.line,
                        column: entity.source_location.column,
                        start_byte: entity.source_location.start_byte,
                        end_byte: entity.source_location.end_byte,
                        kind: UsageKind::EntityDeclaration,
                        context: format!("{} declaration", entity.kind.as_str()),
                    });
                }
            }
        }
    }

    for axiom in &data.axioms {
        if axiom.subject == target_iri {
            if let Some(doc) =
                document_for_ontology_id(data.documents.as_slice(), &axiom.ontology_id)
            {
                let key = (doc.path.clone(), UsageKind::AxiomSubject, axiom.id.clone());
                if seen.insert(key) {
                    usages.push(usage_from_axiom(
                        doc.path.clone(),
                        target_iri,
                        axiom,
                        UsageKind::AxiomSubject,
                    ));
                }
            }
        }
        if axiom.predicate == target_iri {
            if let Some(doc) =
                document_for_ontology_id(data.documents.as_slice(), &axiom.ontology_id)
            {
                let key =
                    (doc.path.clone(), UsageKind::AxiomPredicate, format!("{}-pred", axiom.id));
                if seen.insert(key) {
                    usages.push(usage_from_axiom(
                        doc.path.clone(),
                        target_iri,
                        axiom,
                        UsageKind::AxiomPredicate,
                    ));
                }
            }
        }
        if axiom.object == target_iri || is_named_ref(&axiom.object, target_iri) {
            if let Some(doc) =
                document_for_ontology_id(data.documents.as_slice(), &axiom.ontology_id)
            {
                let key = (doc.path.clone(), UsageKind::AxiomObject, axiom.id.clone());
                if seen.insert(key) {
                    usages.push(usage_from_axiom(
                        doc.path.clone(),
                        target_iri,
                        axiom,
                        UsageKind::AxiomObject,
                    ));
                }
            }
        }
    }

    for ann in &data.annotations {
        if ann.subject == target_iri {
            if let Some(doc) = document_for_ontology_id(data.documents.as_slice(), &ann.ontology_id)
            {
                let key = (
                    doc.path.clone(),
                    UsageKind::AnnotationSubject,
                    format!(
                        "{}-{}-{}-{}",
                        ann.subject,
                        ann.predicate,
                        ann.source_location.line.unwrap_or(0),
                        ann.source_location.start_byte.unwrap_or(0),
                    ),
                );
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: ann.subject.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: ann.source_location.line,
                        column: ann.source_location.column,
                        start_byte: ann.source_location.start_byte,
                        end_byte: ann.source_location.end_byte,
                        kind: UsageKind::AnnotationSubject,
                        context: format!("annotation subject {}", ann.predicate),
                    });
                }
            }
        }
        if ann.predicate == target_iri {
            if let Some(doc) = document_for_ontology_id(data.documents.as_slice(), &ann.ontology_id)
            {
                let key = (
                    doc.path.clone(),
                    UsageKind::AnnotationPredicate,
                    format!(
                        "{}-{}-{}-pred",
                        ann.subject,
                        ann.predicate,
                        ann.source_location.line.unwrap_or(0),
                    ),
                );
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: ann.subject.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: ann.source_location.line,
                        column: ann.source_location.column,
                        start_byte: ann.source_location.start_byte,
                        end_byte: ann.source_location.end_byte,
                        kind: UsageKind::AnnotationPredicate,
                        context: format!("annotation predicate {}", ann.predicate),
                    });
                }
            }
        }
        if ann.object == target_iri {
            if let Some(doc) = document_for_ontology_id(data.documents.as_slice(), &ann.ontology_id)
            {
                let key = (
                    doc.path.clone(),
                    UsageKind::AnnotationObject,
                    format!(
                        "{}-{}-{}-{}",
                        ann.subject,
                        ann.predicate,
                        ann.source_location.line.unwrap_or(0),
                        ann.source_location.start_byte.unwrap_or(0),
                    ),
                );
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: ann.subject.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: ann.source_location.line,
                        column: ann.source_location.column,
                        start_byte: ann.source_location.start_byte,
                        end_byte: ann.source_location.end_byte,
                        kind: UsageKind::AnnotationObject,
                        context: format!("annotation object {}", ann.predicate),
                    });
                }
            }
        }
    }

    for imp in &data.imports {
        if imp.import_iri == target_iri {
            if let Some(doc) = document_for_ontology_id(data.documents.as_slice(), &imp.ontology_id)
            {
                let key = (doc.path.clone(), UsageKind::Import, imp.import_iri.clone());
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: imp.ontology_id.clone(),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: None,
                        column: None,
                        start_byte: None,
                        end_byte: None,
                        kind: UsageKind::Import,
                        context: format!("import {}", imp.import_iri),
                    });
                }
            }
        }
    }

    for doc in &data.documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            continue;
        }
        let text = match read_source_text(&doc.path, document_overrides) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let namespaces = namespaces_for_text(&text, &doc.namespaces);
        let short = short_name_from_iri(target_iri);
        let angle_needle = format!("<{target_iri}>");
        for (line_idx, line) in text.lines().enumerate() {
            if line.contains(angle_needle.as_str()) {
                let mut search_from = 0usize;
                while let Some(col) = line[search_from..].find(&angle_needle) {
                    let col = search_from + col;
                    if is_token_match_at(line, &angle_needle, col) {
                        let key = (
                            doc.path.clone(),
                            UsageKind::TextReference,
                            format!("{line_idx}-{col}-{angle_needle}"),
                        );
                        if seen.insert(key) {
                            usages.push(text_usage(
                                doc,
                                target_iri,
                                line_idx,
                                col,
                                line,
                                UsageKind::TextReference,
                            ));
                        }
                    }
                    search_from = col + angle_needle.len();
                }
            }
            for (prefix, ns) in &namespaces {
                if target_iri.starts_with(ns) && !prefix.is_empty() {
                    let token = format!("{prefix}:{short}");
                    if !line.contains(&token) {
                        continue;
                    }
                    let mut search_from = 0usize;
                    while let Some(col) = line[search_from..].find(&token) {
                        let col = search_from + col;
                        if is_token_match_at(line, &token, col) {
                            let key = (
                                doc.path.clone(),
                                UsageKind::TextReference,
                                format!("{line_idx}-{col}-{token}"),
                            );
                            if seen.insert(key) {
                                usages.push(text_usage(
                                    doc,
                                    target_iri,
                                    line_idx,
                                    col,
                                    line,
                                    UsageKind::TextReference,
                                ));
                            }
                        }
                        search_from = col + token.len();
                    }
                }
            }
            if let Some(default_ns) = namespaces.get("") {
                if target_iri.starts_with(default_ns.as_str()) {
                    let token = format!(":{short}");
                    if !line.contains(&token) {
                        continue;
                    }
                    let mut search_from = 0usize;
                    while let Some(col) = line[search_from..].find(&token) {
                        let col = search_from + col;
                        if is_token_match_at(line, &token, col) {
                            let key = (
                                doc.path.clone(),
                                UsageKind::TextReference,
                                format!("{line_idx}-{col}-{token}"),
                            );
                            if seen.insert(key) {
                                usages.push(text_usage(
                                    doc,
                                    target_iri,
                                    line_idx,
                                    col,
                                    line,
                                    UsageKind::TextReference,
                                ));
                            }
                        }
                        search_from = col + token.len();
                    }
                }
            }
        }
    }

    // SWRL JSON annotation references (inside string literals).
    for doc in &data.documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            continue;
        }
        let Ok(text) = read_source_text(&doc.path, document_overrides) else {
            continue;
        };
        for (rule_idx, rule) in
            strixonomy_swrl::rules_from_turtle_document(&text).into_iter().enumerate()
        {
            if rule.referenced_iris().iter().any(|iri| iri == target_iri) {
                let key = (doc.path.clone(), UsageKind::SwrlReference, format!("swrl-{rule_idx}"));
                if seen.insert(key) {
                    usages.push(Usage {
                        iri: rule.id.clone().unwrap_or_else(|| format!("rule-{rule_idx}")),
                        referenced_iri: target_iri.to_string(),
                        file: doc.path.clone(),
                        line: None,
                        column: None,
                        start_byte: None,
                        end_byte: None,
                        kind: UsageKind::SwrlReference,
                        context: format!("SWRL rule {}", rule.id.as_deref().unwrap_or("anonymous")),
                    });
                }
            }
        }
    }

    usages.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.unwrap_or(0).cmp(&b.line.unwrap_or(0)))
            .then(a.column.unwrap_or(0).cmp(&b.column.unwrap_or(0)))
    });
    usages
}

fn text_usage(
    doc: &strixonomy_core::OntologyDocument,
    target_iri: &str,
    line_idx: usize,
    col: usize,
    line: &str,
    kind: UsageKind,
) -> Usage {
    Usage {
        iri: doc.id.clone(),
        referenced_iri: target_iri.to_string(),
        file: doc.path.clone(),
        line: Some((line_idx + 1) as u64),
        column: Some(col as u64),
        start_byte: None,
        end_byte: None,
        kind,
        context: line.trim().to_string(),
    }
}

fn usage_from_axiom(
    path: PathBuf,
    target_iri: &str,
    axiom: &strixonomy_core::Axiom,
    kind: UsageKind,
) -> Usage {
    Usage {
        iri: axiom.subject.clone(),
        referenced_iri: target_iri.to_string(),
        file: path,
        line: axiom.source_location.line,
        column: axiom.source_location.column,
        start_byte: axiom.source_location.start_byte,
        end_byte: axiom.source_location.end_byte,
        kind,
        context: format!("{} {}", axiom.axiom_kind, axiom.object),
    }
}

fn is_named_ref(object: &str, target_iri: &str) -> bool {
    object == target_iri || object == format!("<{target_iri}>")
}

fn document_for_ontology_id<'a>(
    documents: &'a [OntologyDocument],
    ontology_id: &str,
) -> Option<&'a OntologyDocument> {
    documents.iter().find(|d| document_matches_ontology_id(ontology_id, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_boundary_rejects_person_in_person_type() {
        let line = "ex:PersonType a owl:Class .";
        assert!(!is_token_match_at(line, "ex:Person", line.find("ex:Person").unwrap()));
        assert!(is_token_match_at(line, "ex:PersonType", line.find("ex:PersonType").unwrap()));
    }

    #[test]
    fn find_usages_includes_annotation_predicate() {
        // #398
        use strixonomy_catalog::IndexBuilder;

        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("ann.ttl"),
            concat!(
                "@prefix ex: <http://example.org#> .\n",
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
                "ex:seeAlsoProp a owl:AnnotationProperty .\n",
                "ex:Person a owl:Class ;\n",
                "  ex:seeAlsoProp ex:Other .\n",
            ),
        )
        .unwrap();
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
        let pred = "http://example.org#seeAlsoProp";
        let usages = find_usages(&catalog, pred);
        assert!(
            usages.iter().any(|u| matches!(
                u.kind,
                UsageKind::AnnotationPredicate | UsageKind::AxiomPredicate
            )),
            "must include predicate kind, not only declaration: {usages:?}"
        );
    }
}
