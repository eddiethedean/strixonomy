use crate::turtle_lex::{advance_turtle_scan, is_in_comment_or_string_at, TurtleScanState};
use std::collections::BTreeMap;
use strixonomy_core::{Annotation, Axiom, Entity, SourceLocation};

pub fn short_name_from_iri(iri: &str) -> String {
    if let Some((_, name)) = iri.rsplit_once('#') {
        return name.to_string();
    }
    if let Some((_, name)) = iri.rsplit_once('/') {
        return name.to_string();
    }
    iri.to_string()
}

/// Parse Turtle/SPARQL prefix declarations (`@prefix`, `@PREFIX`, `PREFIX`; case-insensitive).
pub fn prefixes_from_turtle(source_text: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in source_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let Some(keyword) = parts.next() else {
            continue;
        };
        if !(keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX")) {
            continue;
        }
        let Some(prefix_raw) = parts.next() else { continue };
        let Some(uri_raw) = parts.next() else { continue };
        let prefix = prefix_raw.trim_end_matches(':').to_string();
        if let Some(uri) = parse_turtle_prefix_iri(uri_raw) {
            map.insert(prefix, uri);
        }
    }
    map
}

fn parse_turtle_prefix_iri(token: &str) -> Option<String> {
    let token = token.trim_end_matches('.');
    if !token.starts_with('<') || !token.ends_with('>') {
        return None;
    }
    let inner = &token[1..token.len() - 1];
    if inner.is_empty() || inner.contains('<') || inner.contains('>') {
        return None;
    }
    Some(inner.to_string())
}

pub fn namespaces_for_text(
    source_text: &str,
    declared: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut merged = declared.clone();
    merged.extend(prefixes_from_turtle(source_text));
    merged
}

fn subject_needles(
    iri: &str,
    _short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut needles = vec![format!("<{iri}>")];
    // Default prefix (`:Local`) — only when the empty prefix is bound and the suffix is PN_LOCAL.
    if let Some(default_ns) = namespaces.get("") {
        if iri.starts_with(default_ns.as_str()) {
            let local = &iri[default_ns.len()..];
            if crate::patch::is_valid_pn_local(local) {
                needles.push(format!(":{local}"));
            }
        }
    }
    // Prefixed CURIE — longest namespace match + valid PN_LOCAL (same rules as write-back).
    if let Some((prefix, ns)) = crate::patch::best_namespace_match(iri, namespaces) {
        let local = &iri[ns.len()..];
        if crate::patch::is_valid_pn_local(local) {
            needles.push(format!("{prefix}:{local}"));
        }
    }
    needles
}

/// Byte offset immediately after a line's content (handles `\n` and `\r\n`).
fn line_byte_offset_after(text: &str, line_start: usize, line: &str) -> usize {
    let after_content = line_start + line.len();
    let bytes = text.as_bytes();
    if bytes.get(after_content) == Some(&b'\r') && bytes.get(after_content + 1) == Some(&b'\n') {
        after_content + 2
    } else if bytes.get(after_content) == Some(&b'\n') {
        after_content + 1
    } else {
        after_content
    }
}

/// Locate the byte range of an entity's subject token (first Turtle statement for that subject).
#[allow(dead_code)]
pub fn find_subject_statement(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    all_subject_statements(source_text, iri, short_name, namespaces).into_iter().next()
}

/// All statement start positions for a subject IRI in document order.
pub fn all_subject_statements(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<ByteRange> {
    let needles = subject_needles(iri, short_name, namespaces);
    let mut byte_offset = 0usize;
    let mut out = Vec::new();
    for line in source_text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('@') {
            byte_offset = line_byte_offset_after(source_text, byte_offset, line);
            continue;
        }
        if let Some(col_in_trimmed) = subject_column_at_line_start(trimmed, &needles) {
            let ws = line.len() - trimmed.len();
            let start = byte_offset + ws + col_in_trimmed;
            out.push(ByteRange { start: start as u64, end: start as u64 });
        }
        byte_offset = line_byte_offset_after(source_text, byte_offset, line);
    }
    out
}

/// Full byte ranges for every Turtle statement whose subject is `iri`.
pub fn all_entity_statement_ranges(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<ByteRange> {
    all_subject_statements(source_text, iri, short_name, namespaces)
        .into_iter()
        .filter_map(|start_range| {
            let end = statement_end_byte(source_text, start_range.start as usize)?;
            if end > start_range.start as usize {
                Some(ByteRange { start: start_range.start, end: end as u64 })
            } else {
                None
            }
        })
        .collect()
}

/// Prefer the subject statement that declares a type (`a` / `rdf:type`), e.g. `ex:Foo a owl:Class`.
fn primary_entity_statement_start(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    let starts = all_subject_statements(source_text, iri, short_name, namespaces);
    for start in &starts {
        if let Some(end) = statement_end_byte(source_text, start.start as usize) {
            let stmt = &source_text[start.start as usize..end];
            if crate::turtle_lex::statement_has_type_predicate(stmt) {
                return Some(*start);
            }
        }
    }
    starts.first().copied()
}

/// True when `needle` is the subject at the start of a trimmed Turtle line.
fn subject_column_at_line_start(trimmed: &str, needles: &[String]) -> Option<usize> {
    for needle in needles {
        if !trimmed.starts_with(needle) {
            continue;
        }
        let rest = &trimmed[needle.len()..];
        if rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == ';' || c == '.')
        {
            return Some(0);
        }
    }
    None
}

/// Locate the primary entity block (type declaration) in Turtle source.
pub fn find_entity_block(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> SourceLocation {
    if let Some(range) = primary_entity_statement_start(source_text, iri, short_name, namespaces) {
        let start = range.start as usize;
        let line_idx = source_text[..start].matches('\n').count();
        let line_start = source_text[..start].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let col = start - line_start;
        return SourceLocation {
            line: Some((line_idx + 1) as u64),
            column: Some(col as u64),
            start_byte: Some(range.start),
            end_byte: None,
        };
    }
    SourceLocation::default()
}

/// Fill byte spans for labels, comments, and subclass triples.
pub fn annotate_spans(
    source_text: &str,
    entities: &mut [Entity],
    annotations: &mut [Annotation],
    axioms: &mut [Axiom],
) {
    for entity in entities.iter_mut() {
        if let Some(block) = entity_block_range(source_text, entity, &BTreeMap::new()) {
            if entity.source_location.end_byte.is_none() {
                entity.source_location.end_byte = Some(block.end);
            }
        }
    }

    for ann in annotations.iter_mut() {
        if let Some(span) = find_predicate_literal_span(
            source_text,
            &ann.subject,
            predicate_local_name(&ann.predicate),
            &ann.object,
        ) {
            ann.source_location = span;
        }
    }

    for axiom in axioms.iter_mut() {
        if axiom.axiom_kind == strixonomy_core::AXIOM_KIND_SUB_CLASS_OF {
            if let Some(span) = find_subclass_span(source_text, &axiom.subject, &axiom.object) {
                axiom.source_location = span;
            }
        }
    }
}

pub fn entity_block_range(
    source_text: &str,
    entity: &Entity,
    declared_namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    let namespaces = namespaces_for_text(source_text, declared_namespaces);
    let start = if let Some(s) = entity.source_location.start_byte {
        s as usize
    } else {
        primary_entity_statement_start(source_text, &entity.iri, &entity.short_name, &namespaces)
            .map(|r| r.start as usize)?
    };
    let end = statement_end_byte(source_text, start)?;
    if end > start {
        Some(ByteRange { start: start as u64, end: end as u64 })
    } else {
        None
    }
}

/// Primary declaration block for patch insertions (type statement, not trailing triples).
pub fn entity_primary_block_range(
    source_text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<ByteRange> {
    let short_name = short_name_from_iri(entity_iri);
    let start = primary_entity_statement_start(source_text, entity_iri, &short_name, namespaces)?;
    let end = statement_end_byte(source_text, start.start as usize)?;
    if end > start.start as usize {
        Some(ByteRange { start: start.start, end: end as u64 })
    } else {
        None
    }
}

/// True when `b` can appear inside a Turtle PN_LOCAL / prefixed-name token.
pub(crate) fn is_turtle_name_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b':' | b'~' | b'%' | b'\\' | b'.')
}

/// True when `.` at `i` is a statement/object terminator, not part of PN_LOCAL or an IRI.
///
/// Turtle allows `.` inside local names (`ex:foo.bar`) and absolute IRIs (`<http://a.b/c>`).
/// A terminating `.` is never both preceded and followed by name characters.
pub(crate) fn is_turtle_terminating_dot(bytes: &[u8], i: usize) -> bool {
    if bytes.get(i) != Some(&b'.') {
        return false;
    }
    let prev_name = i > 0 && is_turtle_name_char(bytes[i - 1]) && bytes[i - 1] != b'.';
    let next_name = bytes.get(i + 1).is_some_and(|b| is_turtle_name_char(*b) && *b != b'.');
    !(prev_name && next_name)
}

/// True when `byte_offset` lies inside a `#` line comment or any Turtle string literal.
///
/// Handles `"…"`, `'…'`, `"""…"""`, and `'''…'''` forms (same lexer rules as
/// [`statement_end_byte`]).
pub fn is_in_comment_or_string(text: &str, byte_offset: usize) -> bool {
    is_in_comment_or_string_at(text, byte_offset)
}

/// Walk from subject start to the terminating `.` of this statement.
///
/// Tracks `"…"`, `'…'`, `"""…"""`, `'''…'''` strings, IRIs (`<...>`), `#` line comments,
/// brackets, and parens. Does not treat `.` inside IRIs, comments, strings, or PN_LOCAL
/// names (`ex:foo.bar`) as terminators.
pub(crate) fn statement_end_byte(source_text: &str, start: usize) -> Option<usize> {
    let bytes = source_text.as_bytes();
    if start >= bytes.len() {
        return None;
    }
    let mut i = start;
    let mut bracket_depth = 0i32;
    let mut paren_depth = 0i32;
    let mut state = TurtleScanState::default();

    while i < bytes.len() {
        if state.in_comment {
            if bytes[i] == b'\n' {
                state.in_comment = false;
            }
            i += 1;
            continue;
        }
        if state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }

        if bytes.get(i..i + 3) == Some(br#"""""#) {
            state.string_kind = Some(crate::turtle_lex::TurtleStringKind::LongDouble);
            i += 3;
            continue;
        }
        if bytes.get(i..i + 3) == Some(br"'''") {
            state.string_kind = Some(crate::turtle_lex::TurtleStringKind::LongSingle);
            i += 3;
            continue;
        }

        let b = bytes[i];
        match b {
            b'#' => {
                state.in_comment = true;
                i += 1;
            }
            b'"' => {
                state.string_kind = Some(crate::turtle_lex::TurtleStringKind::ShortDouble);
                i += 1;
            }
            b'\'' => {
                state.string_kind = Some(crate::turtle_lex::TurtleStringKind::ShortSingle);
                i += 1;
            }
            b'<' => {
                state.in_iri = true;
                i += 1;
            }
            b'[' => {
                bracket_depth += 1;
                i += 1;
            }
            b']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                i += 1;
            }
            b'(' => {
                paren_depth += 1;
                i += 1;
            }
            b')' => {
                paren_depth = paren_depth.saturating_sub(1);
                i += 1;
            }
            b'.' if bracket_depth == 0
                && paren_depth == 0
                && is_turtle_terminating_dot(bytes, i) =>
            {
                return Some(i + 1);
            }
            _ => i += 1,
        }
    }
    None
}

#[derive(Debug, Clone, Copy)]
pub struct ByteRange {
    pub start: u64,
    pub end: u64,
}

fn line_start_byte(source_text: &str, line_idx: usize) -> u64 {
    let mut offset = 0usize;
    for (i, line) in source_text.lines().enumerate() {
        if i == line_idx {
            return offset as u64;
        }
        offset = line_byte_offset_after(source_text, offset, line);
    }
    0
}

fn predicate_local_name(predicate: &str) -> &str {
    if predicate.contains("label") {
        "label"
    } else if predicate.contains("comment") {
        "comment"
    } else if predicate.contains("subClassOf") {
        "subClassOf"
    } else {
        predicate
    }
}

fn find_predicate_literal_span(
    source_text: &str,
    subject: &str,
    predicate: &str,
    object: &str,
) -> Option<SourceLocation> {
    let object_needle = object.trim_matches('"');
    for (line_idx, line) in source_text.lines().enumerate() {
        if !line.contains(subject) && !line.contains(&short_name_from_iri(subject)) {
            continue;
        }
        if line.contains(predicate) && line.contains(object_needle) {
            let col = line.find(predicate).unwrap_or(0);
            let start_byte = line_start_byte(source_text, line_idx) + col as u64;
            return Some(SourceLocation {
                line: Some((line_idx + 1) as u64),
                column: Some(col as u64),
                start_byte: Some(start_byte),
                end_byte: Some(start_byte + line.len() as u64),
            });
        }
    }
    None
}

fn find_subclass_span(source_text: &str, subject: &str, parent: &str) -> Option<SourceLocation> {
    let parent_short = short_name_from_iri(parent);
    for (line_idx, line) in source_text.lines().enumerate() {
        if !line.contains("subClassOf") {
            continue;
        }
        if (line.contains(subject) || line.contains(&short_name_from_iri(subject)))
            && (line.contains(parent) || line.contains(&parent_short))
        {
            let start_byte = line_start_byte(source_text, line_idx);
            return Some(SourceLocation {
                line: Some((line_idx + 1) as u64),
                column: Some(0),
                start_byte: Some(start_byte),
                end_byte: Some(start_byte + line.len() as u64),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use strixonomy_core::{Entity, EntityKind};

    fn ex_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".into(), "http://example.org/people#".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ])
    }

    fn clinic_ns() -> BTreeMap<String, String> {
        BTreeMap::from([("ex".into(), "http://example.org/clinic#".into())])
    }

    #[test]
    fn finds_entity_line() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let loc = find_entity_block(ttl, "http://example.org/people#Person", "Person", &ex_ns());
        assert!(loc.line.is_some());
        let line = ttl.lines().nth(loc.line.unwrap() as usize - 1).unwrap();
        assert!(line.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn subject_not_domain_mention() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let range =
            find_subject_statement(ttl, "http://example.org/people#Person", "Person", &ex_ns())
                .expect("subject");
        let line = ttl[..range.start as usize].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let subject_line = &ttl[line..];
        assert!(subject_line.starts_with("ex:Person a"));
    }

    #[test]
    fn patient_block_includes_multiline_restriction() {
        let ttl = include_str!("../../../fixtures/complex-classes.ttl");
        let entity = Entity {
            iri: "http://example.org/clinic#Patient".into(),
            short_name: "Patient".into(),
            kind: EntityKind::Class,
            ontology_id: String::new(),
            source_location: find_entity_block(
                ttl,
                "http://example.org/clinic#Patient",
                "Patient",
                &clinic_ns(),
            ),
            labels: vec![],
            comments: vec![],
            deprecated: false,
            obo_id: None,
            ..Default::default()
        };
        let range = entity_block_range(ttl, &entity, &clinic_ns()).expect("block");
        let block = &ttl[range.start as usize..range.end as usize];
        assert!(block.contains("owl:Restriction"));
        assert!(block.contains("owl:someValuesFrom"));
        assert!(block.trim_end().ends_with('.'));
    }

    #[test]
    fn add_label_targets_class_block_not_trailing_triple() {
        use crate::patch::{apply_patches_to_text, PatchOp};
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/people#Person".into(),
            value: "Human".into(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        let person_block_start = preview.find("ex:Person a owl:Class").expect("class decl");
        let human_pos = preview.find("Human").expect("label");
        assert!(human_pos > person_block_start);
        let trailing = preview.find("ex:Person rdfs:subClassOf ex:Thing");
        if let Some(t) = trailing {
            assert!(human_pos < t, "label must be in class block, not trailing triple");
        }
    }

    #[test]
    fn statement_end_respects_dots_in_absolute_iris() {
        let ttl = "<http://example.org/people#Person> a owl:Class ;\n    rdfs:label \"Person\" .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        let block = &ttl[..end];
        assert!(block.contains("rdfs:label"));
        assert!(block.trim_end().ends_with('.'));
        assert!(!block.ends_with("example."));
    }

    #[test]
    fn statement_end_respects_dots_in_comments() {
        let ttl = "ex:Person a owl:Class ; # see docs.\n    rdfs:label \"Person\" .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        let block = &ttl[..end];
        assert!(block.contains("rdfs:label"));
        assert!(block.contains("# see docs."));
    }

    #[test]
    fn statement_end_respects_dots_in_local_names() {
        let ttl = "ex:foo.bar a owl:Class .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        assert_eq!(&ttl[..end], "ex:foo.bar a owl:Class .");
    }

    #[test]
    fn is_in_comment_or_string_detects_all_turtle_string_forms() {
        let double = "rdfs:label \"http://example.org#Person\" .";
        let person_in_double = double.find("http://example.org#Person").unwrap();
        assert!(is_in_comment_or_string(double, person_in_double));

        let single = "rdfs:comment 'http://example.org#Person' .";
        let person_in_single = single.find("http://example.org#Person").unwrap();
        assert!(is_in_comment_or_string(single, person_in_single));

        let long_double = "rdfs:comment \"\"\"http://example.org#Person\"\"\" .";
        let person_in_long_double = long_double.find("http://example.org#Person").unwrap();
        assert!(is_in_comment_or_string(long_double, person_in_long_double));

        let long_single = "rdfs:comment '''http://example.org#Person''' .";
        let person_in_long_single = long_single.find("http://example.org#Person").unwrap();
        assert!(is_in_comment_or_string(long_single, person_in_long_single));

        let outside = "ex:Person a owl:Class .";
        let person_outside = outside.find("ex:Person").unwrap();
        assert!(!is_in_comment_or_string(outside, person_outside));
    }

    #[test]
    fn statement_end_respects_dots_in_long_strings() {
        let ttl = "ex:Person a owl:Class ;\n    rdfs:comment '''See Dr. Smith.''' .\n";
        let end = statement_end_byte(ttl, 0).expect("end");
        let block = &ttl[..end];
        assert!(block.contains("Dr. Smith"));
        assert!(block.trim_end().ends_with('.'));
    }

    #[test]
    fn subject_needles_use_longest_namespace_local_not_short_name() {
        let ns = BTreeMap::from([("ex".into(), "http://example.org/".into())]);
        let iri = "http://example.org/foo/Person";
        let needles = subject_needles(iri, "Person", &ns);
        assert!(needles.contains(&format!("<{iri}>")));
        assert!(
            !needles.iter().any(|n| n == "ex:Person"),
            "must not emit CURIE that expands to a different IRI: {needles:?}"
        );
        // `foo/Person` is not a valid PN_LOCAL, so no prefixed needle.
        assert!(!needles.iter().any(|n| n.starts_with("ex:")));
    }

    #[test]
    fn subject_needles_prefer_longest_prefix_match() {
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("foo".into(), "http://example.org/foo/".into()),
        ]);
        let iri = "http://example.org/foo/Person";
        let needles = subject_needles(iri, "Person", &ns);
        assert!(needles.contains(&"foo:Person".to_string()));
        assert!(
            !needles.iter().any(|n| n == "ex:Person"),
            "shorter prefix must not produce a colliding CURIE: {needles:?}"
        );
    }

    #[test]
    fn find_subject_does_not_match_wrong_curie_under_short_prefix() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
<http://example.org/foo/Person> a owl:Class .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let range = find_subject_statement(ttl, "http://example.org/foo/Person", "Person", &ns)
            .expect("nested path subject");
        let line_start = ttl[..range.start as usize].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let subject_line = &ttl[line_start..];
        assert!(
            subject_line.trim_start().starts_with("<http://example.org/foo/Person>"),
            "must locate the absolute IRI subject, not ex:Person: {subject_line}"
        );
    }

    #[test]
    fn statement_end_includes_single_quoted_label() {
        let ttl = r#"ex:Person a owl:Class ;
    rdfs:label 'Human' ."#;
        let start = ttl.find("ex:Person").unwrap();
        let end = statement_end_byte(ttl, start).expect("end");
        assert!(ttl[start..end].contains("'Human'"));
    }

    #[test]
    fn statement_end_respects_escaped_triple_quotes_in_long_string() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:A a owl:Class ;
  rdfs:comment """contains \""" quote""" .
"#;
        let start = ttl.find("ex:A").unwrap();
        let end = statement_end_byte(ttl, start).expect("end");
        assert!(ttl[start..end].contains(r#"\""" quote"""#));
    }

    #[test]
    fn prefixes_from_turtle_accepts_prefix_keyword() {
        let ttl = "PREFIX ex: <http://example.org/people#>\n";
        let map = prefixes_from_turtle(ttl);
        assert_eq!(map.get("ex"), Some(&"http://example.org/people#".to_string()));
    }

    #[test]
    fn prefixes_from_turtle_accepts_uppercase_at_prefix() {
        let ttl = "@PREFIX owl: <http://www.w3.org/2002/07/owl#> .\n";
        let map = prefixes_from_turtle(ttl);
        assert_eq!(map.get("owl"), Some(&"http://www.w3.org/2002/07/owl#".to_string()));
    }

    #[test]
    fn prefixes_from_turtle_scans_after_prefix_form_and_default_prefix() {
        let ttl = concat!(
            "PREFIX ex: <http://example.org/people#>\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "@prefix : <http://example.org/org#> .\n",
            ":Person a owl:Class .\n"
        );
        let map = prefixes_from_turtle(ttl);
        assert_eq!(map.get("ex"), Some(&"http://example.org/people#".to_string()));
        assert_eq!(map.get("owl"), Some(&"http://www.w3.org/2002/07/owl#".to_string()));
        assert_eq!(map.get(""), Some(&"http://example.org/org#".to_string()));
    }
}
