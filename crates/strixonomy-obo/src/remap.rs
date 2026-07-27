//! Rewrite OBO term/typedef identifiers across a document (v0.24 multi-format refactor).

use crate::error::{OboError, Result};
use std::collections::BTreeSet;

/// Remap `from_id` → `to_id` in an OBO document (id lines and common reference tags).
///
/// Handles `id:`, `is_a:`, `intersection_of:`, `union_of:`, `disjoint_from:`,
/// `relationship:` (second token), `inverse_of:` / `domain:` / `range:` (typedef),
/// and `replaced_by:` / `consider:` fields.
/// Validates the result with fastobo and rejects duplicate `id:` values.
/// Preserves the document's dominant newline style (LF vs CRLF).
pub fn remap_obo_id_in_text(text: &str, from_id: &str, to_id: &str) -> Result<String> {
    remap_obo_id_in_text_inner(text, from_id, to_id, RemapIdMode::Rename)
}

/// Merge `merge_id` into `keep_id`: remove the merge stanza, then remap remaining references.
///
/// Matches Turtle merge semantics (delete merge-subject statements, then rewrite refs).
/// Does not rewrite other stanzas' `id:` lines onto `keep_id`.
pub fn merge_obo_id_in_text(text: &str, merge_id: &str, keep_id: &str) -> Result<String> {
    if merge_id == keep_id {
        return Ok(text.to_string());
    }
    if merge_id.is_empty() || keep_id.is_empty() {
        return Err(OboError::PatchInvalid("empty OBO id".into()));
    }
    let without_merge = match remove_stanza_with_id(text, merge_id) {
        Ok(t) => t,
        Err(OboError::TermNotFound(_)) => text.to_string(),
        Err(e) => return Err(e),
    };
    remap_obo_id_in_text_inner(&without_merge, merge_id, keep_id, RemapIdMode::RefsOnly)
}

/// Replace references to `from_id` with `to_id` without rewriting any `id:` line.
///
/// Matches Turtle replace-when-target-exists: source declaration/stanza is preserved;
/// only reference tags (`is_a:`, `relationship:`, …) are remapped.
pub fn replace_obo_id_refs_in_text(text: &str, from_id: &str, to_id: &str) -> Result<String> {
    remap_obo_id_in_text_inner(text, from_id, to_id, RemapIdMode::RefsOnly)
}

#[derive(Clone, Copy)]
enum RemapIdMode {
    /// Rewrite `id:` and reference tags (rename).
    Rename,
    /// Rewrite reference tags only (merge after stanza removal / replace-when-target-exists).
    RefsOnly,
}

/// Dominant newline sequence in `text` (`\r\n` if present, else `\n`).
fn detect_newline(text: &str) -> &'static str {
    if text.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

fn remap_obo_id_in_text_inner(
    text: &str,
    from_id: &str,
    to_id: &str,
    mode: RemapIdMode,
) -> Result<String> {
    if from_id == to_id {
        return Ok(text.to_string());
    }
    if from_id.is_empty() || to_id.is_empty() {
        return Err(OboError::PatchInvalid("empty OBO id".into()));
    }
    let nl = detect_newline(text);
    let mut out = String::with_capacity(text.len());
    if text.is_empty() {
        // fall through to validate
    } else {
        for piece in text.split_inclusive('\n') {
            let has_ending = piece.ends_with('\n');
            let body = piece.trim_end_matches(['\n', '\r']);
            out.push_str(&remap_obo_line(body, from_id, to_id, mode));
            if has_ending {
                out.push_str(nl);
            }
        }
    }
    assert_unique_obo_ids(&out)?;
    fastobo::from_str(&out).map_err(|e| OboError::Parse(e.to_string()))?;
    Ok(out)
}

fn remap_obo_line(line: &str, from: &str, to: &str, mode: RemapIdMode) -> String {
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];

    let Some((tag, rest)) = trimmed.split_once(':') else {
        return line.to_string();
    };
    let tag_l = tag.trim().to_ascii_lowercase();
    let rest = rest.trim_start();

    let rewritten = match tag_l.as_str() {
        "id" => match mode {
            RemapIdMode::Rename => remap_first_id_token(rest, from, to),
            RemapIdMode::RefsOnly => None,
        },
        // #374: typedef reference tags
        "is_a" | "disjoint_from" | "replaced_by" | "consider" | "union_of" | "inverse_of"
        | "domain" | "range" => remap_first_id_token(rest, from, to),
        "intersection_of" => remap_intersection_of(rest, from, to),
        "relationship" => remap_relationship(rest, from, to),
        _ => None,
    };

    match rewritten {
        Some(new_rest) => format!("{indent}{tag}: {new_rest}"),
        None => line.to_string(),
    }
}

fn remap_first_id_token(rest: &str, from: &str, to: &str) -> Option<String> {
    let (token, rem) = split_obo_token(rest)?;
    if token != from {
        return None;
    }
    Some(if rem.is_empty() { to.to_string() } else { format!("{to}{rem}") })
}

fn remap_intersection_of(rest: &str, from: &str, to: &str) -> Option<String> {
    // Forms: "ID" or "REL ID" optionally followed by " ! comment"
    let (first, after_first) = split_obo_token(rest)?;
    if first == from {
        return Some(if after_first.is_empty() {
            to.to_string()
        } else {
            format!("{to}{after_first}")
        });
    }
    let trimmed = after_first.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('!') {
        return None;
    }
    let (second, after_second) = split_obo_token(trimmed)?;
    if second != from {
        return None;
    }
    Some(format!(
        "{first} {to}{}",
        if after_second.is_empty() { String::new() } else { after_second.to_string() }
    ))
}

fn remap_relationship(rest: &str, from: &str, to: &str) -> Option<String> {
    // relationship: REL ID ! comment
    let (rel, after_rel) = split_obo_token(rest)?;
    let trimmed = after_rel.trim_start();
    let (id, after_id) = split_obo_token(trimmed)?;
    // Remap either relation type or target id when they match `from`.
    let new_rel = if rel == from { to } else { rel.as_str() };
    let new_id = if id == from { to } else { id.as_str() };
    if new_rel == rel && new_id == id {
        return None;
    }
    Some(format!(
        "{new_rel} {new_id}{}",
        if after_id.is_empty() { String::new() } else { after_id.to_string() }
    ))
}

fn split_obo_token(s: &str) -> Option<(String, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    let end = s.find(|c: char| c.is_whitespace() || c == '!').unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    Some((s[..end].to_string(), &s[end..]))
}

fn assert_unique_obo_ids(text: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for line in text.lines() {
        let Some(id) = obo_id_line_token(line) else {
            continue;
        };
        if !seen.insert(id.to_string()) {
            return Err(OboError::PatchInvalid(format!("duplicate OBO id after remap: {id}")));
        }
    }
    Ok(())
}

fn obo_id_line_token(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if !trimmed.to_ascii_lowercase().starts_with("id:") {
        return None;
    }
    let rest = trimmed["id:".len()..].trim_start();
    let end = rest.find(|c: char| c.is_whitespace() || c == '!').unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    Some(&rest[..end])
}

/// Remove the stanza whose `id:` equals `term_id` (including `[Term]`/`[Typedef]`/`[Instance]` header).
fn remove_stanza_with_id(text: &str, term_id: &str) -> Result<String> {
    let id_line_start = find_term_id_line_start(text, term_id)
        .ok_or_else(|| OboError::TermNotFound(term_id.to_string()))?;
    let block_start = preceding_stanza_header(text, id_line_start).unwrap_or(id_line_start);
    let block_end = next_stanza_offset(text, id_line_start).unwrap_or(text.len());
    let mut out = String::with_capacity(text.len().saturating_sub(block_end - block_start));
    out.push_str(&text[..block_start]);
    out.push_str(&text[block_end..]);
    Ok(out)
}

fn preceding_stanza_header(text: &str, before: usize) -> Option<usize> {
    let mut last = None;
    let mut offset = 0usize;
    for line in text[..before].split_inclusive('\n') {
        if is_obo_stanza_header(line.trim_end_matches(['\n', '\r'])) {
            last = Some(offset);
        }
        offset += line.len();
    }
    last
}

fn next_stanza_offset(text: &str, from: usize) -> Option<usize> {
    let mut offset = from;
    let mut skip_current_line = true;
    for line in text[from..].split_inclusive('\n') {
        if skip_current_line {
            skip_current_line = false;
            offset += line.len();
            continue;
        }
        if is_obo_stanza_header(line.trim_end_matches(['\n', '\r'])) {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

fn is_obo_stanza_header(line: &str) -> bool {
    let trimmed = line.trim();
    let Some(name) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return false;
    };
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphabetic())
}

fn find_term_id_line_start(text: &str, term_id: &str) -> Option<usize> {
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        let line_body = line.trim_end_matches(['\n', '\r']);
        if obo_id_line_token(line_body) == Some(term_id) {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaps_id_and_is_a() {
        let src = "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: Person
is_a: EX:000 ! root

[Term]
id: EX:002
name: Child
is_a: EX:001 ! Person
";
        let out = remap_obo_id_in_text(src, "EX:001", "EX:010").expect("remap");
        assert!(out.contains("id: EX:010"));
        assert!(out.contains("is_a: EX:010"));
        assert!(!out.contains("id: EX:001"));
        assert!(out.contains("id: EX:002"));
        assert_eq!(count_id_lines(&out, "EX:010"), 1);
    }

    #[test]
    fn rename_onto_existing_id_fails_closed() {
        let src = "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Term]
id: EX:002
name: B
";
        let err = remap_obo_id_in_text(src, "EX:001", "EX:002").expect_err("duplicate");
        assert!(err.to_string().contains("duplicate OBO id"), "{err}");
    }

    #[test]
    fn merge_removes_merge_stanza_and_rewrites_refs() {
        let src = "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Term]
id: EX:002
name: B
is_a: EX:001

[Term]
id: EX:003
name: C
is_a: EX:001
";
        let out = merge_obo_id_in_text(src, "EX:001", "EX:002").expect("merge");
        assert_eq!(count_id_lines(&out, "EX:002"), 1, "duplicate keep id: {out}");
        assert!(!out.contains("id: EX:001"), "{out}");
        assert!(!out.contains("name: A"), "merge stanza must be removed: {out}");
        assert!(out.contains("id: EX:002"));
        assert!(out.contains("id: EX:003"));
        assert!(out.contains("is_a: EX:002"), "{out}");
        assert!(!out.contains("is_a: EX:001"), "{out}");
    }

    #[test]
    fn replace_refs_preserves_source_id_stanza() {
        let src = "\
format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: From

[Term]
id: EX:002
name: To
is_a: EX:001 ! From
";
        let out = replace_obo_id_refs_in_text(src, "EX:001", "EX:002").expect("replace");
        assert!(out.contains("id: EX:001"), "source id must remain: {out}");
        assert!(out.contains("id: EX:002"));
        assert_eq!(count_id_lines(&out, "EX:001"), 1);
        assert_eq!(count_id_lines(&out, "EX:002"), 1);
        assert!(out.contains("is_a: EX:002"), "{out}");
        assert!(!out.contains("is_a: EX:001"), "{out}");
        assert!(out.contains("name: From"));
    }

    fn count_id_lines(text: &str, id: &str) -> usize {
        text.lines().filter(|l| obo_id_line_token(l) == Some(id)).count()
    }

    #[test]
    fn remaps_typedef_inverse_domain_range() {
        // #374
        let src = "\
format-version: 1.2
ontology: ex

[Typedef]
id: EX:rel
name: related_to
inverse_of: EX:001
domain: EX:001
range: EX:002

[Term]
id: EX:001
name: A

[Term]
id: EX:002
name: B
";
        let out = remap_obo_id_in_text(src, "EX:001", "EX:010").expect("remap");
        assert!(out.contains("id: EX:010"), "{out}");
        assert!(out.contains("inverse_of: EX:010"), "{out}");
        assert!(out.contains("domain: EX:010"), "{out}");
        assert!(!out.contains("inverse_of: EX:001"), "{out}");
        assert!(!out.contains("domain: EX:001"), "{out}");
        assert!(out.contains("range: EX:002"), "{out}");
    }

    #[test]
    fn remap_preserves_crlf_newlines() {
        // #387
        let src = "format-version: 1.2\r\nontology: ex\r\n\r\n[Term]\r\nid: EX:001\r\nname: A\r\n";
        let out = remap_obo_id_in_text(src, "EX:001", "EX:010").expect("remap");
        assert!(out.contains("\r\n"), "must keep CRLF: {out:?}");
        assert!(!out.replace("\r\n", "").contains('\n') || out.contains("\r\n"));
        assert!(out.contains("id: EX:010"));
        // Every newline should be CRLF, not bare LF.
        assert_eq!(out.matches("\r\n").count(), out.matches('\n').count());
    }
}
