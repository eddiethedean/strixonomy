use crate::error::{RefactorError, Result};
use crate::model::{FileChange, Hunk, RefactorPlan};
use crate::source::read_source_text;
use crate::text::{normalize_namespace_base, remap_iri, replace_iri_in_text};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::{validate_workspace_scope_any, EntityKind, OntologyFormat, ParseStatus};

pub fn preview_rename_iri(
    catalog: &OntologyCatalog,
    from_iri: &str,
    to_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    if from_iri == to_iri {
        return Err(RefactorError::Invalid("from and to IRI must differ".to_string()));
    }
    if catalog.find_entity(from_iri).is_none()
        && find_usages_in_catalog(catalog, from_iri, document_overrides).is_empty()
    {
        return Err(RefactorError::EntityNotFound(from_iri.to_string()));
    }

    let mut file_changes: BTreeMap<PathBuf, FileChange> = BTreeMap::new();
    let mut warnings = Vec::new();
    let from_obo = catalog.find_entity(from_iri).and_then(|e| e.obo_id.clone());
    let to_obo = catalog.find_entity(to_iri).and_then(|e| e.obo_id.clone()).or_else(|| {
        // Derive local id from IRIs when renaming within the same OBO namespace.
        short_obo_id_from_iri(to_iri).or_else(|| {
            from_obo.as_ref().map(|_| {
                short_obo_id_from_iri(to_iri).unwrap_or_else(|| {
                    to_iri.rsplit(['#', '/']).next().unwrap_or(to_iri).to_string()
                })
            })
        })
    });

    for doc in &catalog.data().documents {
        if doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, from_iri, document_overrides) {
                warnings.push(format!("skipping errored file: {}", doc.path.display()));
            }
            continue;
        }
        let original = read_source_text(&doc.path, document_overrides)?;
        let preview = match doc.format {
            OntologyFormat::Turtle => {
                if !original.contains(from_iri)
                    && !contains_prefixed_ref(&original, from_iri, &doc.namespaces)
                {
                    continue;
                }
                let (mut preview_text, raw_hunks) =
                    replace_iri_in_text(&original, from_iri, to_iri, &doc.namespaces);
                let (swrl_text, swrl_hits) =
                    strixonomy_swrl::rewrite_swrl_iris_in_turtle(&preview_text, from_iri, to_iri);
                if swrl_hits > 0 {
                    preview_text = swrl_text;
                    warnings.push(format!(
                        "rewrote {swrl_hits} SWRL rule(s) in {}",
                        doc.path.display()
                    ));
                }
                if preview_text == original {
                    continue;
                }
                let hunks: Vec<Hunk> = raw_hunks
                    .into_iter()
                    .map(|(start, end, old_text, new_text)| Hunk {
                        start_byte: start as u64,
                        end_byte: end as u64,
                        old_text,
                        new_text,
                    })
                    .collect();
                file_changes.insert(
                    doc.path.clone(),
                    FileChange {
                        path: doc.path.clone(),
                        preview_text,
                        original_text: original,
                        hunks,
                    },
                );
                continue;
            }
            OntologyFormat::Owl | OntologyFormat::RdfXml => {
                if !original.contains(from_iri) {
                    continue;
                }
                match strixonomy_owl::remap_entity_iri_in_xml_text(
                    &original,
                    "rdfxml",
                    from_iri,
                    to_iri,
                    &doc.namespaces,
                ) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping RDF/XML {}: {e}", doc.path.display()));
                        continue;
                    }
                }
            }
            OntologyFormat::OwlXml => {
                if !original.contains(from_iri) {
                    continue;
                }
                match strixonomy_owl::remap_entity_iri_in_xml_text(
                    &original,
                    "owlxml",
                    from_iri,
                    to_iri,
                    &doc.namespaces,
                ) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping OWL/XML {}: {e}", doc.path.display()));
                        continue;
                    }
                }
            }
            OntologyFormat::Obo => {
                let from_id =
                    from_obo.clone().or_else(|| short_obo_id_from_iri(from_iri)).unwrap_or_else(
                        || from_iri.rsplit(['#', '/']).next().unwrap_or(from_iri).to_string(),
                    );
                let to_id =
                    to_obo.clone().or_else(|| short_obo_id_from_iri(to_iri)).unwrap_or_else(|| {
                        to_iri.rsplit(['#', '/']).next().unwrap_or(to_iri).to_string()
                    });
                if !original.contains(&from_id) {
                    continue;
                }
                match strixonomy_obo::remap_obo_id_in_text(&original, &from_id, &to_id) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping OBO {}: {e}", doc.path.display()));
                        continue;
                    }
                }
            }
            other => {
                if text_contains_iri(doc, from_iri, document_overrides) {
                    warnings.push(format!(
                        "skipping unsupported format {} in {}",
                        other.as_str(),
                        doc.path.display()
                    ));
                }
                continue;
            }
        };

        if preview == original {
            continue;
        }
        file_changes
            .insert(doc.path.clone(), whole_file_change(doc.path.clone(), original, preview));
    }

    if file_changes.is_empty() {
        warnings.push(format!("no files changed for IRI {from_iri}"));
    }

    Ok(RefactorPlan {
        changes: file_changes.into_values().collect(),
        warnings,
        ..Default::default()
    }
    .with_metrics([from_iri, to_iri]))
}

fn short_obo_id_from_iri(iri: &str) -> Option<String> {
    // Common OBO IRIs: http://purl.obolibrary.org/obo/EX_001 → EX:001
    let local = iri.rsplit(['#', '/']).next()?;
    if local.contains('_') && !local.contains(':') {
        let (ns, id) = local.split_once('_')?;
        return Some(format!("{ns}:{id}"));
    }
    if local.contains(':') {
        return Some(local.to_string());
    }
    None
}

pub fn preview_merge_entities(
    catalog: &OntologyCatalog,
    keep_iri: &str,
    merge_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    if keep_iri == merge_iri {
        return Err(RefactorError::Invalid("keep and merge IRI must differ".to_string()));
    }

    let mut warnings = Vec::new();
    if catalog.find_entity(keep_iri).is_none() {
        warnings.push(format!("keep entity not found: {keep_iri}"));
    }
    if catalog.find_entity(merge_iri).is_none() {
        warnings.push(format!("merge entity not found: {merge_iri}"));
    }

    let mut changes = BTreeMap::new();
    for doc in &catalog.data().documents {
        if doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, merge_iri, document_overrides) {
                warnings.push(format!("skipping errored file: {}", doc.path.display()));
            }
            continue;
        }

        let original = read_source_text(&doc.path, document_overrides)?;
        let preview_text = match doc.format {
            OntologyFormat::Turtle => {
                if !original.contains(merge_iri)
                    && !contains_prefixed_ref(&original, merge_iri, &doc.namespaces)
                {
                    continue;
                }
                let namespaces = strixonomy_owl::namespaces_for_text(&original, &doc.namespaces);
                let short = strixonomy_owl::short_name_from_iri(merge_iri);
                let mut declaration_ranges = strixonomy_owl::all_entity_statement_ranges(
                    &original,
                    merge_iri,
                    &short,
                    &namespaces,
                );
                declaration_ranges.sort_by_key(|range| std::cmp::Reverse(range.start));

                let mut without_merge_declaration = original.clone();
                for range in declaration_ranges {
                    without_merge_declaration
                        .replace_range(range.start as usize..range.end as usize, "");
                }
                let (mut preview_text, _) = replace_iri_in_text(
                    &without_merge_declaration,
                    merge_iri,
                    keep_iri,
                    &doc.namespaces,
                );
                let (swrl_text, swrl_hits) = strixonomy_swrl::rewrite_swrl_iris_in_turtle(
                    &preview_text,
                    merge_iri,
                    keep_iri,
                );
                if swrl_hits > 0 {
                    preview_text = swrl_text;
                    warnings.push(format!(
                        "rewrote {swrl_hits} SWRL rule(s) in {}",
                        doc.path.display()
                    ));
                }
                preview_text
            }
            OntologyFormat::Owl | OntologyFormat::RdfXml | OntologyFormat::OwlXml => {
                if !original.contains(merge_iri) {
                    continue;
                }
                let fmt = if doc.format == OntologyFormat::OwlXml { "owlxml" } else { "rdfxml" };
                // Delete merge-owned axioms then remap refs (#369) — matches Turtle/OBO.
                match strixonomy_owl::merge_entity_iri_in_xml_text(
                    &original,
                    fmt,
                    keep_iri,
                    merge_iri,
                    &doc.namespaces,
                ) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping {} {}: {e}", fmt, doc.path.display()));
                        continue;
                    }
                }
            }
            OntologyFormat::Obo => {
                let from_id = catalog
                    .find_entity(merge_iri)
                    .and_then(|e| e.obo_id.clone())
                    .or_else(|| short_obo_id_from_iri(merge_iri))
                    .unwrap_or_else(|| {
                        merge_iri.rsplit(['#', '/']).next().unwrap_or(merge_iri).to_string()
                    });
                let to_id = catalog
                    .find_entity(keep_iri)
                    .and_then(|e| e.obo_id.clone())
                    .or_else(|| short_obo_id_from_iri(keep_iri))
                    .unwrap_or_else(|| {
                        keep_iri.rsplit(['#', '/']).next().unwrap_or(keep_iri).to_string()
                    });
                if !original.contains(&from_id) {
                    continue;
                }
                // Delete merge stanza then remap refs (#367) — matches Turtle merge.
                match strixonomy_obo::merge_obo_id_in_text(&original, &from_id, &to_id) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping OBO {}: {e}", doc.path.display()));
                        continue;
                    }
                }
            }
            other => {
                if text_contains_iri(doc, merge_iri, document_overrides) {
                    warnings.push(format!(
                        "skipping unsupported format {} in {}",
                        other.as_str(),
                        doc.path.display()
                    ));
                }
                continue;
            }
        };

        if preview_text == original {
            continue;
        }

        changes
            .insert(doc.path.clone(), whole_file_change(doc.path.clone(), original, preview_text));
    }

    if changes.is_empty() {
        warnings.push(format!("no files changed for entity merge {merge_iri} -> {keep_iri}"));
    }

    Ok(RefactorPlan { changes: changes.into_values().collect(), warnings, ..Default::default() }
        .with_metrics([keep_iri, merge_iri]))
}

pub fn preview_replace_entity(
    catalog: &OntologyCatalog,
    from_iri: &str,
    to_iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    if catalog.find_entity(to_iri).is_none() {
        return preview_rename_iri(catalog, from_iri, to_iri, document_overrides);
    }
    if from_iri == to_iri {
        return Err(RefactorError::Invalid("from and to IRI must differ".to_string()));
    }
    if catalog.find_entity(from_iri).is_none()
        && find_usages_in_catalog(catalog, from_iri, document_overrides).is_empty()
    {
        return Err(RefactorError::EntityNotFound(from_iri.to_string()));
    }

    let mut changes = BTreeMap::new();
    let mut warnings = Vec::new();
    for doc in &catalog.data().documents {
        if doc.parse_status != ParseStatus::Ok {
            if text_contains_iri(doc, from_iri, document_overrides) {
                warnings.push(format!("skipping errored file: {}", doc.path.display()));
            }
            continue;
        }

        let original = read_source_text(&doc.path, document_overrides)?;
        let preview_text = match doc.format {
            OntologyFormat::Turtle => {
                if !original.contains(from_iri)
                    && !contains_prefixed_ref(&original, from_iri, &doc.namespaces)
                {
                    continue;
                }
                let namespaces = strixonomy_owl::namespaces_for_text(&original, &doc.namespaces);
                let declaration_start =
                    strixonomy_owl::entity_primary_block_range(&original, from_iri, &namespaces)
                        .map(|range| range.start as usize);
                let mut preview_text = replace_iri_preserving_subject(
                    &original,
                    from_iri,
                    to_iri,
                    &doc.namespaces,
                    declaration_start,
                );
                let (swrl_text, swrl_hits) =
                    strixonomy_swrl::rewrite_swrl_iris_in_turtle(&preview_text, from_iri, to_iri);
                if swrl_hits > 0 {
                    preview_text = swrl_text;
                    warnings.push(format!(
                        "rewrote {swrl_hits} SWRL rule(s) in {}",
                        doc.path.display()
                    ));
                }
                preview_text
            }
            OntologyFormat::Owl | OntologyFormat::RdfXml | OntologyFormat::OwlXml => {
                if !original.contains(from_iri) {
                    continue;
                }
                // Existing-target replace on XML remaps all mentions (incl. declaration).
                warnings.push(format!(
                    "non-Turtle replace rewrites all IRI mentions in {}",
                    doc.path.display()
                ));
                let fmt = if doc.format == OntologyFormat::OwlXml { "owlxml" } else { "rdfxml" };
                match strixonomy_owl::remap_entity_iri_in_xml_text(
                    &original,
                    fmt,
                    from_iri,
                    to_iri,
                    &doc.namespaces,
                ) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping {} {}: {e}", fmt, doc.path.display()));
                        continue;
                    }
                }
            }
            OntologyFormat::Obo => {
                let from_id = catalog
                    .find_entity(from_iri)
                    .and_then(|e| e.obo_id.clone())
                    .or_else(|| short_obo_id_from_iri(from_iri))
                    .unwrap_or_else(|| {
                        from_iri.rsplit(['#', '/']).next().unwrap_or(from_iri).to_string()
                    });
                let to_id = catalog
                    .find_entity(to_iri)
                    .and_then(|e| e.obo_id.clone())
                    .or_else(|| short_obo_id_from_iri(to_iri))
                    .unwrap_or_else(|| {
                        to_iri.rsplit(['#', '/']).next().unwrap_or(to_iri).to_string()
                    });
                if !original.contains(&from_id) {
                    continue;
                }
                // Preserve source `id:` stanza; remap refs only (#367) — matches Turtle.
                match strixonomy_obo::replace_obo_id_refs_in_text(&original, &from_id, &to_id) {
                    Ok(text) => text,
                    Err(e) => {
                        warnings.push(format!("skipping OBO {}: {e}", doc.path.display()));
                        continue;
                    }
                }
            }
            other => {
                if text_contains_iri(doc, from_iri, document_overrides) {
                    warnings.push(format!(
                        "skipping unsupported format {} in {}",
                        other.as_str(),
                        doc.path.display()
                    ));
                }
                continue;
            }
        };

        if preview_text == original {
            continue;
        }

        changes
            .insert(doc.path.clone(), whole_file_change(doc.path.clone(), original, preview_text));
    }

    if changes.is_empty() {
        warnings.push(format!("no files changed for entity replacement {from_iri} -> {to_iri}"));
    }

    Ok(RefactorPlan { changes: changes.into_values().collect(), warnings, ..Default::default() }
        .with_metrics([from_iri, to_iri]))
}

fn replace_iri_preserving_subject(
    text: &str,
    from_iri: &str,
    to_iri: &str,
    namespaces: &BTreeMap<String, String>,
    subject_start: Option<usize>,
) -> String {
    let Some(subject_start) = subject_start else {
        return replace_iri_in_text(text, from_iri, to_iri, namespaces).0;
    };
    let subject_text = &text[subject_start..];
    let subject_end = if subject_text.starts_with('<') {
        subject_text.find('>').map(|offset| subject_start + offset + 1).unwrap_or(text.len())
    } else {
        subject_text
            .find(|c: char| c.is_whitespace() || c == ';')
            .map(|offset| subject_start + offset)
            .unwrap_or(text.len())
    };

    let before = replace_iri_in_text(&text[..subject_start], from_iri, to_iri, namespaces).0;
    let after = replace_iri_in_text(&text[subject_end..], from_iri, to_iri, namespaces).0;
    format!("{before}{}{after}", &text[subject_start..subject_end])
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

pub fn preview_migrate_namespace(
    catalog: &OntologyCatalog,
    from_base: &str,
    to_base: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Result<RefactorPlan> {
    let from = normalize_namespace_base(from_base);
    let to = normalize_namespace_base(to_base);
    if from == to {
        return Err(RefactorError::Invalid("from and to namespace must differ".to_string()));
    }

    let all_iris: Vec<String> = catalog
        .data()
        .entities
        .iter()
        .filter(|e| remap_iri(&e.iri, &from, &to).is_some())
        .map(|e| e.iri.clone())
        .chain(catalog.data().axioms.iter().flat_map(|a| {
            let mut v = Vec::new();
            if remap_iri(&a.subject, &from, &to).is_some() {
                v.push(a.subject.clone());
            }
            if remap_iri(&a.object, &from, &to).is_some() {
                v.push(a.object.clone());
            }
            v
        }))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let mut changes: BTreeMap<PathBuf, FileChange> = BTreeMap::new();
    let mut warnings = Vec::new();

    for doc in &catalog.data().documents {
        if doc.format != OntologyFormat::Turtle || doc.parse_status != ParseStatus::Ok {
            continue;
        }
        let original = read_source_text(&doc.path, document_overrides)?;
        let mut preview = original.clone();
        let mut hunks = Vec::new();
        let mut changed = false;

        for old_iri in &all_iris {
            let new_iri = remap_iri(old_iri, &from, &to).unwrap_or_else(|| old_iri.clone());
            if !preview.contains(old_iri)
                && !contains_prefixed_ref(&preview, old_iri, &doc.namespaces)
            {
                continue;
            }
            let (next, raw_hunks) =
                replace_iri_in_text(&preview, old_iri, &new_iri, &doc.namespaces);
            if next != preview {
                preview = next;
                changed = true;
                hunks.extend(raw_hunks.into_iter().map(|(s, e, o, n)| Hunk {
                    start_byte: s as u64,
                    end_byte: e as u64,
                    old_text: o,
                    new_text: n,
                }));
            }
        }

        for (prefix, ns) in &doc.namespaces {
            if normalize_namespace_base(ns) == from {
                let terminator = if ns.ends_with('/') { '/' } else { '#' };
                let new_ns = format!("{to}{terminator}");
                let (next, raw_hunks) = replace_prefix_uri(&preview, prefix, ns, &new_ns);
                if next != preview {
                    preview = next;
                    changed = true;
                    hunks.extend(raw_hunks.into_iter().map(|(s, e, o, n)| Hunk {
                        start_byte: s as u64,
                        end_byte: e as u64,
                        old_text: o,
                        new_text: n,
                    }));
                }
            }
        }

        if changed {
            changes.insert(
                doc.path.clone(),
                FileChange {
                    path: doc.path.clone(),
                    preview_text: preview,
                    original_text: original,
                    hunks,
                },
            );
        }
    }

    if changes.is_empty() {
        warnings.push(format!("no Turtle files changed for namespace migration {from} -> {to}"));
    }

    Ok(RefactorPlan { changes: changes.into_values().collect(), warnings, ..Default::default() }
        .with_metrics([from, to]))
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

fn replace_prefix_uri(
    text: &str,
    prefix: &str,
    old_uri: &str,
    new_uri: &str,
) -> (String, Vec<(usize, usize, String, String)>) {
    let old_term = format!("<{old_uri}>");
    let new_term = format!("<{new_uri}>");
    let mut out = String::with_capacity(text.len());
    let mut hunks = Vec::new();
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        let updated = if prefix_declaration_name(line) == Some(prefix) && line.contains(&old_term) {
            line.replacen(&old_term, &new_term, 1)
        } else {
            line.to_string()
        };
        if updated != line {
            hunks.push((offset, offset + line.len(), line.to_string(), updated.clone()));
        }
        out.push_str(&updated);
        offset += line.len();
    }
    (out, hunks)
}

fn escape_turtle_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn find_usages_in_catalog(
    catalog: &OntologyCatalog,
    iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> Vec<()> {
    let u = crate::usages::find_usages_with_overrides(catalog, iri, document_overrides);
    vec![(); u.len()]
}

fn text_contains_iri(
    doc: &strixonomy_core::OntologyDocument,
    iri: &str,
    document_overrides: &HashMap<PathBuf, String>,
) -> bool {
    read_source_text(&doc.path, document_overrides).map(|t| t.contains(iri)).unwrap_or(false)
}

fn contains_prefixed_ref(text: &str, iri: &str, namespaces: &BTreeMap<String, String>) -> bool {
    let short = strixonomy_owl::short_name_from_iri(iri);
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && text.contains(&format!("{prefix}:{short}")) {
            return true;
        }
    }
    false
}

fn prefixed_curie(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    let short = strixonomy_owl::short_name_from_iri(iri);
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            return format!("{prefix}:{short}");
        }
    }
    format!("<{iri}>")
}

fn owl_type_for_kind(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Class => "owl:Class",
        EntityKind::ObjectProperty => "owl:ObjectProperty",
        EntityKind::DataProperty => "owl:DatatypeProperty",
        EntityKind::AnnotationProperty => "owl:AnnotationProperty",
        EntityKind::Individual => "owl:NamedIndividual",
        EntityKind::Datatype => "rdfs:Datatype",
        EntityKind::Ontology => "owl:Ontology",
        EntityKind::Other => "owl:Class",
    }
}

struct EntityRemoval {
    path: PathBuf,
    start: u64,
    end: u64,
    replacement: String,
}

/// Preview a refactor. Client-supplied paths (`target_file` / `output_file`) are jailed under
/// `workspace_roots` **before** any filesystem read.
pub fn preview_refactor(
    catalog: &OntologyCatalog,
    request: &crate::model::RefactorRequest,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    match request {
        crate::model::RefactorRequest::RenameIri { from_iri, to_iri } => {
            preview_rename_iri(catalog, from_iri, to_iri, document_overrides)
        }
        crate::model::RefactorRequest::MergeEntities { keep_iri, merge_iri } => {
            preview_merge_entities(catalog, keep_iri, merge_iri, document_overrides)
        }
        crate::model::RefactorRequest::ReplaceEntity { from_iri, to_iri } => {
            preview_replace_entity(catalog, from_iri, to_iri, document_overrides)
        }
        crate::model::RefactorRequest::MigrateNamespace { from_base, to_base } => {
            preview_migrate_namespace(catalog, from_base, to_base, document_overrides)
        }
        crate::model::RefactorRequest::MoveEntity { entity_iri, target_file } => {
            preview_move_entity(
                catalog,
                entity_iri,
                target_file,
                document_overrides,
                workspace_roots,
            )
        }
        crate::model::RefactorRequest::ExtractModule {
            entity_iris,
            output_file,
            leave_stub,
            locality,
        } => preview_extract_module(
            catalog,
            entity_iris,
            output_file,
            *leave_stub,
            *locality,
            document_overrides,
            workspace_roots,
        ),
        crate::model::RefactorRequest::MoveAxioms {
            entity_iri,
            target_file,
            statement_indexes,
            exclude_primary,
        } => preview_move_axioms(
            catalog,
            entity_iri,
            target_file,
            statement_indexes,
            *exclude_primary,
            document_overrides,
            workspace_roots,
        ),
        crate::model::RefactorRequest::MergeOntologies { source_paths, target_file } => {
            crate::ontology::preview_merge_ontologies(
                catalog,
                source_paths,
                target_file,
                document_overrides,
                workspace_roots,
            )
        }
        crate::model::RefactorRequest::FlattenImports { ontology_file } => {
            crate::ontology::preview_flatten_imports(
                catalog,
                ontology_file,
                document_overrides,
                workspace_roots,
            )
        }
        crate::model::RefactorRequest::CleanupImports { ontology_file } => {
            crate::ontology::preview_cleanup_imports(
                catalog,
                ontology_file,
                document_overrides,
                workspace_roots,
            )
        }
    }
}

fn require_path_in_workspace(path: &Path, workspace_roots: &[PathBuf]) -> Result<()> {
    validate_workspace_scope_any(path, workspace_roots).map_err(RefactorError::Invalid)?;
    Ok(())
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub fn preview_move_entity(
    catalog: &OntologyCatalog,
    entity_iri: &str,
    target_file: &Path,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    // Jail before any filesystem read of the client-supplied path.
    require_path_in_workspace(target_file, workspace_roots)?;
    catalog
        .find_entity(entity_iri)
        .ok_or_else(|| RefactorError::EntityNotFound(entity_iri.to_string()))?;
    let source_doc = catalog
        .entity_document(entity_iri)
        .ok_or_else(|| RefactorError::Invalid(format!("no document for {entity_iri}")))?;
    if source_doc.format != OntologyFormat::Turtle {
        return Err(RefactorError::UnsupportedFormat(source_doc.format.as_str().to_string()));
    }

    let source_canon = canonical_path(&source_doc.path);
    let target_canon = canonical_path(target_file);
    if source_canon == target_canon {
        return Err(RefactorError::Invalid(
            "target file must differ from source document".to_string(),
        ));
    }

    let source_text = read_source_text(&source_doc.path, document_overrides)?;
    let namespaces = strixonomy_owl::namespaces_for_text(&source_text, &source_doc.namespaces);
    let short = strixonomy_owl::short_name_from_iri(entity_iri);
    let mut ranges =
        strixonomy_owl::all_entity_statement_ranges(&source_text, entity_iri, &short, &namespaces);
    if ranges.is_empty() {
        return Err(RefactorError::Invalid(format!("entity block not found for {entity_iri}")));
    }
    ranges.sort_by_key(|r| r.start);

    let mut block_parts = Vec::new();
    let mut hunks = Vec::new();
    for range in &ranges {
        let part = source_text[range.start as usize..range.end as usize].to_string();
        block_parts.push(part.clone());
        hunks.push(Hunk {
            start_byte: range.start,
            end_byte: range.end,
            old_text: part,
            new_text: String::new(),
        });
    }
    let block_text = block_parts.join("\n");
    let mut source_without = source_text.clone();
    for range in ranges.into_iter().rev() {
        source_without.replace_range(range.start as usize..range.end as usize, "");
    }

    let mut prefix_lines = BTreeSet::new();
    for line in source_text.lines() {
        if is_prefix_declaration_line(line) {
            prefix_lines.insert(line.trim().to_string());
        }
    }
    let prefix_header: String = prefix_lines.iter().cloned().collect::<Vec<_>>().join("\n");
    let block_with_prefixes = if prefix_header.is_empty() {
        block_text.clone()
    } else {
        format!("{prefix_header}\n\n{block_text}")
    };

    let target_original = if target_file.exists() {
        read_source_text(target_file, document_overrides)?
    } else {
        String::new()
    };
    let target_was_empty = target_original.is_empty();
    let mut warnings = Vec::new();
    let (target_preview, target_hunk_new) = if target_was_empty {
        let mut out = block_with_prefixes.clone();
        if !out.ends_with('\n') {
            out.push('\n');
        }
        (out, block_with_prefixes)
    } else {
        // Merge missing source @prefix bindings into non-empty targets (#314).
        let target_prefix_names: BTreeSet<String> = target_original
            .lines()
            .filter_map(prefix_declaration_name)
            .map(str::to_string)
            .collect();
        let mut missing_prefixes = Vec::new();
        for line in &prefix_lines {
            let Some(name) = prefix_declaration_name(line) else {
                continue;
            };
            if !target_prefix_names.contains(name) {
                missing_prefixes.push(line.clone());
            }
        }
        let inserted = if missing_prefixes.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", missing_prefixes.join("\n"))
        };
        if !missing_prefixes.is_empty() {
            warnings.push(format!(
                "added {} missing @prefix declaration(s) to {}",
                missing_prefixes.len(),
                target_file.display()
            ));
        }
        let hunk_new = format!("{inserted}{block_text}");
        (format!("{target_original}\n\n{hunk_new}"), hunk_new)
    };

    Ok(RefactorPlan {
        changes: vec![
            FileChange {
                path: source_doc.path.clone(),
                preview_text: source_without,
                original_text: source_text,
                hunks,
            },
            FileChange {
                path: target_file.to_path_buf(),
                preview_text: target_preview,
                original_text: target_original,
                hunks: vec![Hunk {
                    start_byte: 0,
                    end_byte: 0,
                    old_text: String::new(),
                    new_text: target_hunk_new,
                }],
            },
        ],
        warnings,
        ..Default::default()
    }
    .with_metrics([entity_iri]))
}

/// Move selected subject statements for `entity_iri` into `target_file`.
pub fn preview_move_axioms(
    catalog: &OntologyCatalog,
    entity_iri: &str,
    target_file: &Path,
    statement_indexes: &[usize],
    exclude_primary: bool,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    require_path_in_workspace(target_file, workspace_roots)?;
    catalog
        .find_entity(entity_iri)
        .ok_or_else(|| RefactorError::EntityNotFound(entity_iri.to_string()))?;
    let Some(source_doc) = catalog.entity_document(entity_iri) else {
        return Err(RefactorError::EntityNotFound(entity_iri.to_string()));
    };
    if source_doc.format != OntologyFormat::Turtle || source_doc.parse_status != ParseStatus::Ok {
        return Err(RefactorError::Invalid(
            "move axioms currently supports Turtle documents only".to_string(),
        ));
    }
    let source_canon = canonical_path(&source_doc.path);
    let target_canon = canonical_path(target_file);
    if source_canon == target_canon {
        return Err(RefactorError::Invalid(
            "target file must differ from the source document".to_string(),
        ));
    }

    let source_text = read_source_text(&source_doc.path, document_overrides)?;
    let namespaces = strixonomy_owl::namespaces_for_text(&source_text, &source_doc.namespaces);
    let short = strixonomy_owl::short_name_from_iri(entity_iri);
    let ranges =
        strixonomy_owl::all_entity_statement_ranges(&source_text, entity_iri, &short, &namespaces);
    if ranges.is_empty() {
        return Err(RefactorError::Invalid(format!(
            "no Turtle statements found for entity {entity_iri}"
        )));
    }

    let primary = strixonomy_owl::entity_primary_block_range(&source_text, entity_iri, &namespaces);
    let selectable: Vec<(usize, strixonomy_owl::ByteRange)> = ranges
        .into_iter()
        .enumerate()
        .filter(|(_, range)| {
            if !exclude_primary {
                return true;
            }
            primary.map(|p| p.start != range.start || p.end != range.end).unwrap_or(true)
        })
        .collect();

    if selectable.is_empty() {
        return Err(RefactorError::Invalid(
            "no movable axiom statements (only primary declaration remains)".to_string(),
        ));
    }

    let chosen: Vec<strixonomy_owl::ByteRange> = if statement_indexes.is_empty() {
        selectable.into_iter().map(|(_, r)| r).collect()
    } else {
        let mut out = Vec::new();
        for idx in statement_indexes {
            let Some((_, range)) = selectable.iter().find(|(i, _)| *i == *idx) else {
                return Err(RefactorError::Invalid(format!(
                    "statement index {idx} out of range for entity {entity_iri}"
                )));
            };
            out.push(*range);
        }
        out
    };

    let mut warnings = Vec::new();
    let mut blocks = Vec::new();
    let mut source_without = source_text.clone();
    let mut ordered = chosen;
    ordered.sort_by_key(|r| std::cmp::Reverse(r.start));
    for range in &ordered {
        let block = source_text[range.start as usize..range.end as usize].trim().to_string();
        if !block.is_empty() {
            blocks.push(block);
        }
        source_without.replace_range(range.start as usize..range.end as usize, "");
    }
    while source_without.contains("\n\n\n") {
        source_without = source_without.replace("\n\n\n", "\n\n");
    }
    blocks.reverse();
    let moved_text = blocks.join("\n\n");
    if moved_text.is_empty() {
        return Err(RefactorError::Invalid("selected statements produced empty move".to_string()));
    }

    let target_original = if target_file.exists() {
        read_source_text(target_file, document_overrides)?
    } else {
        String::new()
    };
    let prefix_lines: Vec<String> = source_text
        .lines()
        .filter(|line| is_prefix_declaration_line(line))
        .map(str::to_string)
        .collect();
    let target_prefix_names: BTreeSet<String> =
        target_original.lines().filter_map(prefix_declaration_name).map(str::to_string).collect();
    let mut missing_prefixes = Vec::new();
    for line in &prefix_lines {
        let Some(name) = prefix_declaration_name(line) else {
            continue;
        };
        if !target_prefix_names.contains(name) {
            missing_prefixes.push(line.clone());
        }
    }
    let inserted = if missing_prefixes.is_empty() {
        String::new()
    } else {
        warnings.push(format!(
            "added {} missing @prefix declaration(s) to {}",
            missing_prefixes.len(),
            target_file.display()
        ));
        format!("{}\n\n", missing_prefixes.join("\n"))
    };
    let target_preview = if target_original.is_empty() {
        format!("{inserted}{moved_text}\n")
    } else {
        format!("{target_original}\n\n{inserted}{moved_text}\n")
    };

    Ok(RefactorPlan {
        changes: vec![
            FileChange {
                path: source_doc.path.clone(),
                preview_text: source_without,
                original_text: source_text,
                hunks: vec![],
            },
            FileChange {
                path: target_file.to_path_buf(),
                preview_text: target_preview.clone(),
                original_text: target_original,
                hunks: vec![Hunk {
                    start_byte: 0,
                    end_byte: 0,
                    old_text: String::new(),
                    new_text: format!("{inserted}{moved_text}"),
                }],
            },
        ],
        warnings,
        ..Default::default()
    }
    .with_metrics([entity_iri]))
}

pub fn preview_extract_module(
    catalog: &OntologyCatalog,
    entity_iris: &[String],
    output_file: &Path,
    leave_stub: bool,
    locality: bool,
    document_overrides: &HashMap<PathBuf, String>,
    workspace_roots: &[PathBuf],
) -> Result<RefactorPlan> {
    // Jail before any filesystem read of the client-supplied path.
    require_path_in_workspace(output_file, workspace_roots)?;
    if entity_iris.is_empty() {
        return Err(RefactorError::Invalid("no entities selected".to_string()));
    }

    let expanded: Vec<String> = if locality {
        crate::ontology::expand_signature_locality(catalog, entity_iris)
    } else {
        entity_iris.to_vec()
    };
    let entity_iris = &expanded;

    let mut blocks = Vec::new();
    let mut removals: Vec<EntityRemoval> = Vec::new();
    let mut source_texts: BTreeMap<PathBuf, String> = BTreeMap::new();
    let mut prefix_lines = BTreeSet::new();
    let mut warnings = Vec::new();
    if locality {
        warnings.push(format!(
            "locality expansion selected {} entit{}",
            entity_iris.len(),
            if entity_iris.len() == 1 { "y" } else { "ies" }
        ));
    }

    for iri in entity_iris {
        let entity =
            catalog.find_entity(iri).ok_or_else(|| RefactorError::EntityNotFound(iri.clone()))?;
        let doc = catalog
            .entity_document(iri)
            .ok_or_else(|| RefactorError::Invalid(format!("no document for {iri}")))?;
        if doc.format != OntologyFormat::Turtle {
            return Err(RefactorError::UnsupportedFormat(doc.format.as_str().to_string()));
        }
        let text = if let Some(existing) = source_texts.get(&doc.path) {
            existing.clone()
        } else {
            read_source_text(&doc.path, document_overrides)?
        };
        source_texts.insert(doc.path.clone(), text.clone());
        for line in text.lines() {
            if is_prefix_declaration_line(line) {
                prefix_lines.insert(line.trim().to_string());
            }
        }
        let namespaces = strixonomy_owl::namespaces_for_text(&text, &doc.namespaces);
        let short = strixonomy_owl::short_name_from_iri(iri);
        let mut ranges =
            strixonomy_owl::all_entity_statement_ranges(&text, iri, &short, &namespaces);
        if ranges.is_empty() {
            return Err(RefactorError::Invalid(format!("block not found for {iri}")));
        }
        ranges.sort_by_key(|r| r.start);
        let mut entity_blocks = Vec::new();
        for (idx, range) in ranges.iter().enumerate() {
            let block = text[range.start as usize..range.end as usize].to_string();
            entity_blocks.push(block);
            let replacement = if leave_stub && idx == 0 {
                let owl_type = owl_type_for_kind(entity.kind);
                let moved_path = escape_turtle_string(&output_file.display().to_string());
                format!(
                    "{} a {owl_type} ;\n    owl:deprecated true ;\n    rdfs:comment \"Moved to {moved_path}\" .\n",
                    prefixed_curie(iri, &namespaces),
                )
            } else {
                String::new()
            };
            removals.push(EntityRemoval {
                path: doc.path.clone(),
                start: range.start,
                end: range.end,
                replacement,
            });
        }
        blocks.push(entity_blocks.join("\n"));
    }

    let mut source_changes: BTreeMap<PathBuf, (String, String, Vec<Hunk>)> = BTreeMap::new();
    let mut removals_by_path: BTreeMap<PathBuf, Vec<EntityRemoval>> = BTreeMap::new();
    for removal in removals {
        removals_by_path.entry(removal.path.clone()).or_default().push(removal);
    }
    for (path, mut path_removals) in removals_by_path {
        path_removals.sort_by_key(|b| std::cmp::Reverse(b.start));
        let original = source_texts.remove(&path).ok_or_else(|| {
            RefactorError::Invalid(format!("missing source text for {}", path.display()))
        })?;
        let mut preview = original.clone();
        let mut hunks = Vec::new();
        for removal in path_removals {
            let start = removal.start as usize;
            let end = removal.end as usize;
            let old_text = preview[start..end].to_string();
            preview.replace_range(start..end, &removal.replacement);
            hunks.push(Hunk {
                start_byte: removal.start,
                end_byte: removal.end,
                old_text,
                new_text: removal.replacement.clone(),
            });
        }
        source_changes.insert(path, (original, preview, hunks));
    }

    let mut module_body = blocks.join("\n\n");
    if !module_body.ends_with('\n') {
        module_body.push('\n');
    }
    let prefix_header: String = prefix_lines.into_iter().collect::<Vec<_>>().join("\n");
    let module_text = if prefix_header.is_empty() {
        module_body
    } else {
        format!("{prefix_header}\n\n{module_body}")
    };

    let mut changes: Vec<FileChange> = source_changes
        .into_iter()
        .map(|(path, (original, preview, hunks))| FileChange {
            path: path.clone(),
            preview_text: preview,
            original_text: original,
            hunks,
        })
        .collect();

    let output_original = if output_file.exists() {
        read_source_text(output_file, document_overrides)?
    } else {
        String::new()
    };
    let output_preview = if output_original.is_empty() {
        module_text.clone()
    } else {
        format!("{output_original}\n\n{module_text}")
    };
    changes.push(FileChange {
        path: output_file.to_path_buf(),
        preview_text: output_preview,
        original_text: output_original,
        hunks: vec![Hunk {
            start_byte: 0,
            end_byte: 0,
            old_text: String::new(),
            new_text: module_text,
        }],
    });

    if leave_stub {
        warnings.push("left deprecated stubs in source files".to_string());
    }

    Ok(RefactorPlan { changes, warnings, ..Default::default() }
        .with_metrics(entity_iris.iter().map(|s| s.as_str())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_prefix_uri_updates_at_prefix_uppercase() {
        let text = "@PREFIX ex: <http://example.org/org#> .\nex:Person a owl:Class .\n";
        let (out, hunks) =
            replace_prefix_uri(text, "ex", "http://example.org/org#", "http://example.org/v2/org#");
        assert!(out.contains("@PREFIX ex: <http://example.org/v2/org#>"));
        assert!(!out.contains("@PREFIX ex: <http://example.org/org#>"));
        assert!(!hunks.is_empty());
    }

    #[test]
    fn replace_prefix_uri_updates_sparql_style_prefix() {
        let text = "PREFIX ex: <http://example.org/org#>\nex:Person a owl:Class .\n";
        let (out, _) =
            replace_prefix_uri(text, "ex", "http://example.org/org#", "http://example.org/v2/org#");
        assert!(out.contains("PREFIX ex: <http://example.org/v2/org#>"));
    }

    #[test]
    fn owl_type_for_kind_maps_ontology() {
        assert_eq!(owl_type_for_kind(EntityKind::Ontology), "owl:Ontology");
        assert_eq!(owl_type_for_kind(EntityKind::Other), "owl:Class");
        assert_eq!(owl_type_for_kind(EntityKind::Class), "owl:Class");
    }

    #[test]
    fn escape_turtle_string_escapes_path_specials() {
        assert_eq!(
            escape_turtle_string(r#"C:\ontology\mod"ule.ttl"#),
            r#"C:\\ontology\\mod\"ule.ttl"#
        );
    }
}
