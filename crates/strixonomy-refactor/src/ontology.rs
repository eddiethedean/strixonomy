//! Ontology-level Turtle refactor operations (merge, flatten/cleanup imports).
//!
//! # Locality-based module extraction
//!
//! When [`crate::preview_extract_module`] is called with `locality = true`, the seed
//! signature is closed under a **bottom-locality MVP heuristic** before signature-copy
//! extraction:
//!
//! 1. Start with Σ = the requested entity IRIs.
//! 2. Walk catalog axioms; for each axiom α let `sig(α)` be its named entities
//!    (subject, object, and non-builtin predicate), excluding RDF/RDFS/OWL/XSD IRIs.
//! 3. If `sig(α) ⊆ Σ`, treat α as bottom-local and (redundantly) keep Σ unchanged.
//! 4. If `sig(α) ∩ Σ ≠ ∅`, select α and expand `Σ := Σ ∪ sig(α)`.
//! 5. Repeat until Σ is quiet (fixed-point).
//! 6. Extract Turtle statement blocks for every entity remaining in Σ (same move
//!    semantics as signature-copy extract).
//!
//! Steps 3–4 together grow a connected module around the seed; axioms already wholly
//! inside Σ stay available for copy via entity subject blocks.

use crate::error::{RefactorError, Result};
use crate::model::{FileChange, Hunk, RefactorPlan};
use crate::source::read_source_text;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::{
    document_lookup::{document_matches_ontology_id, normalize_iri},
    OntologyFormat, ParseStatus,
};

/// Expand seed entity IRIs under the bottom-locality MVP heuristic (see module docs).
pub fn expand_signature_locality(catalog: &OntologyCatalog, seeds: &[String]) -> Vec<String> {
    let mut sigma: BTreeSet<String> = seeds.iter().cloned().collect();
    let known: BTreeSet<&str> = catalog.data().entities.iter().map(|e| e.iri.as_str()).collect();

    let mut changed = true;
    while changed {
        changed = false;
        for axiom in &catalog.data().axioms {
            let sig = axiom_named_signature(axiom, &known);
            if sig.is_empty() {
                continue;
            }
            let intersects = sig.iter().any(|iri| sigma.contains(iri));
            let subset = sig.iter().all(|iri| sigma.contains(iri));
            if subset {
                // Bottom-local wrt Σ — already covered; no expansion needed.
                continue;
            }
            if intersects {
                for iri in sig {
                    // Only pull catalog-backed entities into the extract set.
                    if !known.contains(iri.as_str()) {
                        continue;
                    }
                    if sigma.insert(iri) {
                        changed = true;
                    }
                }
            }
        }
    }
    sigma.into_iter().collect()
}

fn axiom_named_signature(
    axiom: &strixonomy_core::Axiom,
    known: &BTreeSet<&str>,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for candidate in [&axiom.subject, &axiom.object, &axiom.predicate] {
        if is_builtin_iri(candidate) {
            continue;
        }
        // Prefer catalog entities; still accept absolute http(s) IRIs for external parents.
        if known.contains(candidate.as_str()) || looks_like_absolute_iri(candidate) {
            out.insert(candidate.clone());
        }
    }
    out
}

fn looks_like_absolute_iri(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://") || s.starts_with("urn:")
}

fn is_builtin_iri(iri: &str) -> bool {
    iri.starts_with("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
        || iri.starts_with("http://www.w3.org/2000/01/rdf-schema#")
        || iri.starts_with("http://www.w3.org/2002/07/owl#")
        || iri.starts_with("http://www.w3.org/2001/XMLSchema#")
        || iri == "http://www.w3.org/2002/07/owl#Thing"
        || iri == "http://www.w3.org/2002/07/owl#Nothing"
}

fn require_path_in_workspace(path: &Path, workspace_roots: &[PathBuf]) -> Result<()> {
    strixonomy_core::validate_workspace_scope_any(path, workspace_roots)
        .map_err(RefactorError::Invalid)?;
    Ok(())
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn find_doc_by_path<'a>(
    catalog: &'a OntologyCatalog,
    path: &Path,
) -> Option<&'a strixonomy_core::OntologyDocument> {
    let canon = canonical_path(path);
    catalog.data().documents.iter().find(|d| {
        d.path == path || canonical_path(&d.path) == canon || canonical_path(&d.path) == path
    })
}

fn resolve_import_doc<'a>(
    catalog: &'a OntologyCatalog,
    import_iri: &str,
) -> Option<&'a strixonomy_core::OntologyDocument> {
    let norm = normalize_iri(import_iri);
    if let Some(doc) = catalog.data().documents.iter().find(|d| {
        document_matches_ontology_id(import_iri, d)
            || d.base_iri.as_ref().is_some_and(|b| normalize_iri(b) == norm || b == import_iri)
            || normalize_iri(&d.id) == norm
            || d.id == import_iri
    }) {
        return Some(doc);
    }
    // Protégé catalog-v001.xml / Oasis URI redirects (local only).
    resolve_import_via_xml_catalog(catalog, import_iri)
}

/// Public import resolve: ontology IRI match, then workspace `catalog-v001.xml` redirects.
pub fn resolve_import_document<'a>(
    catalog: &'a OntologyCatalog,
    import_iri: &str,
) -> Option<&'a strixonomy_core::OntologyDocument> {
    resolve_import_doc(catalog, import_iri)
}

fn resolve_import_via_xml_catalog<'a>(
    catalog: &'a OntologyCatalog,
    import_iri: &str,
) -> Option<&'a strixonomy_core::OntologyDocument> {
    use std::collections::BTreeSet;
    let mut roots = BTreeSet::new();
    for doc in &catalog.data().documents {
        if let Some(parent) = doc.path.parent() {
            roots.insert(parent.to_path_buf());
            if let Some(grand) = parent.parent() {
                roots.insert(grand.to_path_buf());
            }
        }
    }
    for root in roots {
        let Ok(xml) = strixonomy_catalog::load_workspace_xml_catalogs(&root) else {
            continue;
        };
        if xml.uri_entries.is_empty() && xml.rewrite_entries.is_empty() {
            continue;
        }
        let Some(target) = xml.resolve(import_iri) else {
            continue;
        };
        if let Some(doc) = find_doc_by_path(catalog, &target) {
            return Some(doc);
        }
        // Match by file name when relative redirect didn't canonicalize
        if let Some(name) = target.file_name() {
            if let Some(doc) =
                catalog.data().documents.iter().find(|d| d.path.file_name() == Some(name))
            {
                return Some(doc);
            }
        }
    }
    None
}

fn is_prefix_declaration_line(line: &str) -> bool {
    let keyword = line.split_whitespace().next().unwrap_or("");
    keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX")
}

fn prefix_declaration_name(line: &str) -> Option<&str> {
    let mut parts = line.split_whitespace();
    let keyword = parts.next()?;
    if !(keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX")) {
        return None;
    }
    parts.next()?.strip_suffix(':')
}

fn whole_file_change(path: PathBuf, original_text: String, preview_text: String) -> FileChange {
    FileChange {
        path,
        hunks: vec![Hunk {
            start_byte: 0,
            end_byte: original_text.len() as u64,
            old_text: original_text.clone(),
            new_text: preview_text.clone(),
        }],
        preview_text,
        original_text,
    }
}

fn split_prefixes_and_body(text: &str) -> (Vec<String>, String) {
    let mut prefixes = Vec::new();
    let mut body_lines = Vec::new();
    let mut in_prefix_header = true;
    for line in text.lines() {
        if in_prefix_header && (is_prefix_declaration_line(line) || line.trim().is_empty()) {
            if is_prefix_declaration_line(line) {
                prefixes.push(line.to_string());
            }
            continue;
        }
        in_prefix_header = false;
        body_lines.push(line);
    }
    let body = body_lines.join("\n").trim().to_string();
    (prefixes, body)
}

fn merge_missing_prefixes(target: &str, source_prefixes: &[String]) -> (String, Vec<String>) {
    let target_names: BTreeSet<String> =
        target.lines().filter_map(prefix_declaration_name).map(str::to_string).collect();
    let mut missing = Vec::new();
    for line in source_prefixes {
        let Some(name) = prefix_declaration_name(line) else {
            continue;
        };
        if !target_names.contains(name) {
            missing.push(line.clone());
        }
    }
    if missing.is_empty() {
        return (target.to_string(), missing);
    }
    // Insert missing prefixes after existing prefix block (or at top).
    let mut out = String::new();
    let mut inserted = false;
    let mut saw_prefix = false;
    for line in target.split_inclusive('\n') {
        if is_prefix_declaration_line(line) {
            saw_prefix = true;
            out.push_str(line);
            continue;
        }
        if saw_prefix && !inserted && !is_prefix_declaration_line(line) {
            for p in &missing {
                out.push_str(p);
                if !p.ends_with('\n') {
                    out.push('\n');
                }
            }
            out.push('\n');
            inserted = true;
        }
        out.push_str(line);
    }
    if !inserted {
        let mut header = missing.join("\n");
        if !header.ends_with('\n') {
            header.push('\n');
        }
        out = format!("{header}\n{target}");
    }
    (out, missing)
}

fn body_already_present(target: &str, body: &str) -> bool {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Exact substring match of the source body is enough for MVP dedupe.
    target.contains(trimmed)
}

fn is_turtle_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("ttl") || e.eq_ignore_ascii_case("turtle"))
}

/// Merge source ontology Turtle documents into `target_file` (prefix union + body append).
pub fn preview_merge_ontologies(
    catalog: &OntologyCatalog,
    source_paths: &[PathBuf],
    target_file: &Path,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    require_path_in_workspace(target_file, workspace_roots)?;
    if source_paths.is_empty() {
        return Err(RefactorError::Invalid("no source ontology paths provided".to_string()));
    }
    for src in source_paths {
        require_path_in_workspace(src, workspace_roots)?;
    }

    let target_canon = canonical_path(target_file);
    let mut warnings = Vec::new();
    let original = if target_file.exists() || document_overrides.contains_key(target_file) {
        read_source_text(target_file, document_overrides)?
    } else {
        String::new()
    };
    let mut preview = original.clone();
    let mut appended_entities = Vec::new();

    for src in source_paths {
        if canonical_path(src) == target_canon {
            warnings.push(format!("skipping source equal to target: {}", src.display()));
            continue;
        }
        if !is_turtle_path(src) {
            warnings.push(format!("skipping non-Turtle source: {}", src.display()));
            continue;
        }
        if let Some(doc) = find_doc_by_path(catalog, src) {
            if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
                warnings.push(format!("skipping non-Turtle or errored file: {}", src.display()));
                continue;
            }
        }

        let source_text = read_source_text(src, document_overrides)?;
        let (prefixes, body) = split_prefixes_and_body(&source_text);
        if body.is_empty() {
            warnings.push(format!("empty body in source: {}", src.display()));
            continue;
        }
        if body_already_present(&preview, &body) {
            warnings.push(format!("source body already present in target: {}", src.display()));
            continue;
        }
        let (with_prefixes, missing) = merge_missing_prefixes(&preview, &prefixes);
        preview = with_prefixes;
        if !missing.is_empty() {
            warnings.push(format!(
                "added {} missing @prefix declaration(s) from {}",
                missing.len(),
                src.display()
            ));
        }
        if preview.trim().is_empty() {
            preview = format!("{body}\n");
        } else {
            let mut next = preview.trim_end().to_string();
            next.push_str("\n\n");
            next.push_str(&body);
            if !body.ends_with('\n') {
                next.push('\n');
            }
            preview = next;
        }
        appended_entities.push(src.display().to_string());
    }

    if preview == original {
        warnings.push("no ontology content merged into target".to_string());
    }

    let changes = if preview == original {
        vec![]
    } else {
        vec![whole_file_change(target_file.to_path_buf(), original, preview)]
    };

    Ok(RefactorPlan { changes, warnings, ..Default::default() }.with_metrics(appended_entities))
}

fn collect_import_closure<'a>(
    catalog: &'a OntologyCatalog,
    root: &'a strixonomy_core::OntologyDocument,
) -> (Vec<&'a strixonomy_core::OntologyDocument>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut visited = BTreeSet::new();
    let mut queue: Vec<String> = root.imports.clone();
    let mut ordered = Vec::new();

    visited.insert(canonical_path(&root.path));

    while let Some(import_iri) = queue.pop() {
        let Some(doc) = resolve_import_doc(catalog, &import_iri) else {
            warnings.push(format!("unresolved import: {import_iri}"));
            continue;
        };
        let canon = canonical_path(&doc.path);
        if !visited.insert(canon) {
            continue;
        }
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            warnings.push(format!(
                "skipping non-Turtle or errored import: {} ({})",
                import_iri,
                doc.path.display()
            ));
            continue;
        }
        for child in &doc.imports {
            queue.push(child.clone());
        }
        ordered.push(doc);
    }
    (ordered, warnings)
}

fn strip_owl_imports_lines(text: &str) -> (String, usize) {
    let mut removed = 0usize;
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim();
        // Match owl:imports / <…owl#imports> predicate lines.
        let is_import = trimmed.contains("owl:imports")
            || trimmed.contains("owl#imports>")
            || trimmed.to_ascii_lowercase().contains("owl:imports");
        if is_import {
            removed += 1;
            continue;
        }
        out.push_str(line);
    }
    // Clean up dangling "a owl:Ontology ;" followed by "." after import removal.
    let cleaned = cleanup_ontology_block_after_import_strip(&out);
    (cleaned, removed)
}

fn cleanup_ontology_block_after_import_strip(text: &str) -> String {
    // Turn `a owl:Ontology ;` / `\n.` sequences into `a owl:Ontology .`
    let mut result = text.to_string();
    // Common pattern: ontology IRI block ending with `;` then blank then `.`
    while result.contains(";\n.") || result.contains(";\r\n.") {
        result = result.replace(";\n.", ".");
        result = result.replace(";\r\n.", ".");
    }
    // `a owl:Ontology ;\n\n` → keep; trailing semicolon before next subject is ok.
    // Collapse `a owl:Ontology ;\n` at end of statement when next non-ws is unrelated:
    result = result.replace("a owl:Ontology ;\n\n", "a owl:Ontology .\n\n");
    result = result.replace("a owl:Ontology ;\n", "a owl:Ontology .\n");
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }
    result
}

fn extract_import_iris_from_text(text: &str) -> Vec<String> {
    let mut iris = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !(trimmed.contains("owl:imports") || trimmed.contains("owl#imports>")) {
            continue;
        }
        // Prefer <IRI> on the import line.
        if let Some(start) = trimmed.rfind('<') {
            if let Some(end) = trimmed[start + 1..].find('>') {
                let iri = &trimmed[start + 1..start + 1 + end];
                if !iri.is_empty() {
                    iris.push(iri.to_string());
                }
            }
        }
    }
    iris
}

/// Inline axioms from imported Turtle documents into the root file and remove imports.
pub fn preview_flatten_imports(
    catalog: &OntologyCatalog,
    ontology_file: &Path,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    require_path_in_workspace(ontology_file, workspace_roots)?;
    let doc = find_doc_by_path(catalog, ontology_file).ok_or_else(|| {
        RefactorError::Invalid(format!("ontology file not in catalog: {}", ontology_file.display()))
    })?;
    if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
        return Err(RefactorError::UnsupportedFormat(doc.format.as_str().to_string()));
    }

    let original = read_source_text(&doc.path, document_overrides)?;
    let mut warnings = Vec::new();
    let (imports, mut import_warnings) = collect_import_closure(catalog, doc);
    warnings.append(&mut import_warnings);

    if imports.is_empty() && doc.imports.is_empty() {
        warnings.push("no owl:imports to flatten".to_string());
        return Ok(RefactorPlan { changes: vec![], warnings, ..Default::default() }
            .with_metrics(Vec::<String>::new()));
    }

    let (mut preview, removed) = strip_owl_imports_lines(&original);
    if removed == 0 && !doc.imports.is_empty() {
        warnings.push(
            "catalog lists imports but no owl:imports lines matched; left stub warnings".into(),
        );
        for iri in &doc.imports {
            warnings.push(format!("# TODO: flatten unresolved/unmatched import {iri}"));
        }
    }

    for imported in imports {
        let source_text = read_source_text(&imported.path, document_overrides)?;
        let (prefixes, body) = split_prefixes_and_body(&source_text);
        if body.is_empty() {
            continue;
        }
        if body_already_present(&preview, &body) {
            warnings.push(format!(
                "imported body already present, skipped: {}",
                imported.path.display()
            ));
            continue;
        }
        let (with_prefixes, missing) = merge_missing_prefixes(&preview, &prefixes);
        preview = with_prefixes;
        if !missing.is_empty() {
            warnings.push(format!(
                "added {} missing @prefix declaration(s) from {}",
                missing.len(),
                imported.path.display()
            ));
        }
        let mut next = preview.trim_end().to_string();
        next.push_str("\n\n");
        next.push_str("# inlined from ");
        next.push_str(&imported.path.display().to_string());
        next.push('\n');
        next.push_str(&body);
        if !body.ends_with('\n') {
            next.push('\n');
        }
        preview = next;
    }

    if preview == original {
        warnings.push("flatten produced no textual changes".to_string());
        return Ok(RefactorPlan { changes: vec![], warnings, ..Default::default() }
            .with_metrics(Vec::<String>::new()));
    }

    Ok(RefactorPlan {
        changes: vec![whole_file_change(doc.path.clone(), original, preview)],
        warnings,
        ..Default::default()
    }
    .with_metrics(doc.imports.clone()))
}

fn entity_iris_for_document(
    catalog: &OntologyCatalog,
    doc: &strixonomy_core::OntologyDocument,
) -> Vec<String> {
    catalog
        .data()
        .entities
        .iter()
        .filter(|e| {
            e.ontology_id == doc.id
                || doc
                    .base_iri
                    .as_ref()
                    .is_some_and(|b| normalize_iri(b) == normalize_iri(&e.ontology_id))
                || catalog
                    .entity_document(&e.iri)
                    .is_some_and(|d| canonical_path(&d.path) == canonical_path(&doc.path))
        })
        .map(|e| e.iri.clone())
        .collect()
}

fn text_references_any_entity(
    text: &str,
    entity_iris: &[String],
    namespaces: &BTreeMap<String, String>,
) -> bool {
    for iri in entity_iris {
        if text.contains(iri) {
            return true;
        }
        let short = strixonomy_owl::short_name_from_iri(iri);
        for (prefix, ns) in namespaces {
            if iri.starts_with(ns.as_str()) {
                let curie = if prefix.is_empty() {
                    format!(":{short}")
                } else {
                    format!("{prefix}:{short}")
                };
                if text.contains(&curie) {
                    return true;
                }
            }
        }
        // Default: angle form
        if text.contains(&format!("<{iri}>")) {
            return true;
        }
    }
    false
}

fn remove_import_lines_for_iri(text: &str, import_iri: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if line.contains(import_iri)
            && (line.contains("owl:imports") || line.contains("owl#imports>"))
        {
            continue;
        }
        out.push_str(line);
    }
    cleanup_ontology_block_after_import_strip(&out)
}

/// Remove unused `owl:imports` when none of the imported ontology's entities are referenced.
pub fn preview_cleanup_imports(
    catalog: &OntologyCatalog,
    ontology_file: &Path,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    require_path_in_workspace(ontology_file, workspace_roots)?;
    let doc = find_doc_by_path(catalog, ontology_file).ok_or_else(|| {
        RefactorError::Invalid(format!("ontology file not in catalog: {}", ontology_file.display()))
    })?;
    if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
        return Err(RefactorError::UnsupportedFormat(doc.format.as_str().to_string()));
    }

    let original = read_source_text(&doc.path, document_overrides)?;
    let namespaces = strixonomy_owl::namespaces_for_text(&original, &doc.namespaces);
    let mut warnings = Vec::new();
    let mut preview = original.clone();
    let mut removed_iris = Vec::new();

    let mut import_iris: BTreeSet<String> = doc.imports.iter().cloned().collect();
    for iri in extract_import_iris_from_text(&original) {
        import_iris.insert(iri);
    }

    for import_iri in import_iris {
        let Some(imported) = resolve_import_doc(catalog, &import_iri) else {
            warnings.push(format!("leaving unresolved import: {import_iri}"));
            continue;
        };
        if imported.format != OntologyFormat::Turtle {
            warnings.push(format!("skipping non-Turtle import for cleanup: {import_iri}"));
            continue;
        }
        let entities = entity_iris_for_document(catalog, imported);
        // Build reference text without the import line itself.
        let without_this_import = remove_import_lines_for_iri(&preview, &import_iri);
        if entities.is_empty() {
            // No indexed entities — treat as unused if the import IRI itself isn't otherwise used.
            warnings.push(format!(
                "imported ontology has no indexed entities; removing unused import {import_iri}"
            ));
            preview = without_this_import;
            removed_iris.push(import_iri);
            continue;
        }
        if text_references_any_entity(&without_this_import, &entities, &namespaces) {
            continue;
        }
        preview = without_this_import;
        removed_iris.push(import_iri);
    }

    if preview == original {
        warnings.push("no unused owl:imports found".to_string());
        return Ok(RefactorPlan { changes: vec![], warnings, ..Default::default() }
            .with_metrics(Vec::<String>::new()));
    }

    if !removed_iris.is_empty() {
        warnings.push(format!("removed {} unused owl:imports", removed_iris.len()));
    }

    Ok(RefactorPlan {
        changes: vec![whole_file_change(doc.path.clone(), original, preview)],
        warnings,
        ..Default::default()
    }
    .with_metrics(removed_iris))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_builtin_recognizes_owl_rdf() {
        assert!(is_builtin_iri("http://www.w3.org/2002/07/owl#Class"));
        assert!(is_builtin_iri("http://www.w3.org/2000/01/rdf-schema#subClassOf"));
        assert!(!is_builtin_iri("http://example.org#Person"));
    }

    #[test]
    fn strip_owl_imports_removes_lines() {
        let text = concat!(
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "<http://example.org/root> a owl:Ontology ;\n",
            "    owl:imports <http://example.org/lib> .\n",
            "ex:A a owl:Class .\n"
        );
        let (out, n) = strip_owl_imports_lines(text);
        assert_eq!(n, 1);
        assert!(!out.contains("owl:imports"));
        assert!(out.contains("ex:A"));
    }
}
