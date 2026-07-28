use crate::error::{OboError, Result};
use crate::patch::OboPatchOp;
use serde::{Deserialize, Serialize};
use std::path::Path;
use strixonomy_core::{read_to_string_capped, MAX_FILE_BYTES};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDiagnostic {
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyPatchResult {
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<PatchDiagnostic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_path: Option<String>,
}

pub fn apply_patches(
    document_path: &Path,
    patches: &[OboPatchOp],
    preview_only: bool,
) -> Result<ApplyPatchResult> {
    let source = read_to_string_capped(document_path, MAX_FILE_BYTES).map_err(OboError::Core)?;
    let mut result = apply_patches_to_text(&source, patches, preview_only)?;
    result.document_path = Some(document_path.display().to_string());
    if result.applied && !preview_only {
        if let Some(text) = &result.preview_text {
            atomic_write(document_path, text)?;
        }
    }
    Ok(result)
}

pub fn apply_patches_to_text(
    source: &str,
    patches: &[OboPatchOp],
    preview_only: bool,
) -> Result<ApplyPatchResult> {
    let mut working = source.to_string();
    let mut diagnostics = Vec::new();
    for patch in patches {
        if let Err(e) = apply_one(&mut working, patch) {
            diagnostics
                .push(PatchDiagnostic { severity: "error".to_string(), message: e.to_string() });
            return Ok(ApplyPatchResult {
                applied: false,
                preview_text: Some(source.to_string()),
                diagnostics,
                document_path: None,
            });
        }
    }
    validate_obo(&working)?;
    let changed = working != source;
    Ok(ApplyPatchResult {
        applied: changed && !preview_only,
        preview_text: if changed { Some(working) } else { None },
        diagnostics,
        document_path: None,
    })
}

fn validate_obo(text: &str) -> Result<()> {
    fastobo::from_str(text).map_err(|e| OboError::Parse(e.to_string()))?;
    Ok(())
}

/// Reject values that would break single-line OBO fields or inject extra stanzas.
fn reject_line_breaks(value: &str, field: &str) -> Result<()> {
    if value.contains('\n') || value.contains('\r') {
        return Err(OboError::PatchInvalid(format!("{field} must not contain line breaks")));
    }
    Ok(())
}

/// Reject OBO token values (IDs, xrefs) that contain whitespace or line breaks.
fn reject_obo_token(value: &str, field: &str) -> Result<()> {
    reject_line_breaks(value, field)?;
    if value.chars().any(char::is_whitespace) {
        return Err(OboError::PatchInvalid(format!("{field} must not contain whitespace")));
    }
    Ok(())
}

fn validate_patch(patch: &OboPatchOp) -> Result<()> {
    match patch {
        OboPatchOp::SetName { term_id, value } => {
            reject_obo_token(term_id, "term_id")?;
            reject_line_breaks(value, "name")?;
        }
        OboPatchOp::AddSynonym { term_id, value, scope } => {
            reject_obo_token(term_id, "term_id")?;
            reject_line_breaks(value, "synonym")?;
            reject_line_breaks(scope, "synonym scope")?;
        }
        OboPatchOp::RemoveSynonym { term_id, value, scope } => {
            reject_obo_token(term_id, "term_id")?;
            reject_line_breaks(value, "synonym")?;
            if let Some(scope) = scope {
                reject_line_breaks(scope, "synonym scope")?;
            }
        }
        OboPatchOp::AddDef { term_id, value } => {
            reject_obo_token(term_id, "term_id")?;
            reject_line_breaks(value, "definition")?;
        }
        OboPatchOp::RemoveDef { term_id } => reject_obo_token(term_id, "term_id")?,
        OboPatchOp::AddXref { term_id, xref } => {
            reject_obo_token(term_id, "term_id")?;
            reject_obo_token(xref, "xref")?;
        }
        OboPatchOp::RemoveXref { term_id, xref } => {
            reject_obo_token(term_id, "term_id")?;
            reject_obo_token(xref, "xref")?;
        }
        OboPatchOp::SetNamespace { term_id, namespace } => {
            reject_obo_token(term_id, "term_id")?;
            reject_line_breaks(namespace, "namespace")?;
        }
        OboPatchOp::SetDeprecated { term_id, .. } => reject_obo_token(term_id, "term_id")?,
        OboPatchOp::AddIsA { term_id, parent_id } => {
            reject_obo_token(term_id, "term_id")?;
            reject_obo_token(parent_id, "parent_id")?;
        }
        OboPatchOp::RemoveIsA { term_id, parent_id } => {
            reject_obo_token(term_id, "term_id")?;
            reject_obo_token(parent_id, "parent_id")?;
        }
    }
    Ok(())
}

fn apply_one(text: &mut String, patch: &OboPatchOp) -> Result<()> {
    validate_patch(patch)?;
    match patch {
        OboPatchOp::SetName { term_id, value } => {
            set_single_line(text, term_id, "name:", value, false)
        }
        OboPatchOp::AddSynonym { term_id, value, scope } => {
            let escaped = value.replace('"', "\\\"");
            let line = format!("synonym: \"{escaped}\" {scope} []");
            add_line_in_term(text, term_id, &line)
        }
        OboPatchOp::RemoveSynonym { term_id, value, scope } => {
            remove_synonym_line(text, term_id, value, scope.as_deref())
        }
        OboPatchOp::AddDef { term_id, value } => {
            let escaped = value.replace('"', "\\\"");
            let line = format!("def: \"{escaped}\" []");
            set_or_add_line(text, term_id, "def:", &line)
        }
        OboPatchOp::RemoveDef { term_id } => remove_lines_with_prefix(text, term_id, "def:"),
        OboPatchOp::AddXref { term_id, xref } => {
            let line = format!("xref: {xref}");
            add_line_in_term(text, term_id, &line)
        }
        OboPatchOp::RemoveXref { term_id, xref } => remove_xref_line(text, term_id, xref),
        OboPatchOp::SetNamespace { term_id, namespace } => {
            set_single_line(text, term_id, "namespace:", namespace, false)
        }
        OboPatchOp::SetDeprecated { term_id, value } => {
            let val = if *value { "true" } else { "false" };
            set_single_line(text, term_id, "is_obsolete:", val, false)
        }
        OboPatchOp::AddIsA { term_id, parent_id } => {
            let line = format!("is_a: {parent_id} ! {parent_id}");
            add_line_in_term(text, term_id, &line)
        }
        OboPatchOp::RemoveIsA { term_id, parent_id } => remove_is_a_line(text, term_id, parent_id),
    }
}

fn term_block_range(text: &str, term_id: &str) -> Result<(usize, usize)> {
    let id_line_start = find_term_id_line_start(text, term_id)
        .ok_or_else(|| OboError::TermNotFound(term_id.to_string()))?;
    let block_start = preceding_stanza_header(text, id_line_start).unwrap_or(id_line_start);
    let block_end = next_stanza_offset(text, id_line_start).unwrap_or(text.len());
    Ok((block_start, block_end))
}

/// Byte offset of the stanza header (`[Term]`, `[Typedef]`, `[Instance]`, …) that owns `before`.
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

/// Dominant newline sequence in `text` (`\r\n` if present, else `\n`).
fn detect_newline(text: &str) -> &'static str {
    if text.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

/// Byte offset of the next OBO stanza header (`[Term]`, `[Typedef]`, `[Instance]`, …)
/// at or after `from`, or `None` if this is the last stanza.
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
        if obo_id_line_matches(line_body, term_id) {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

fn obo_id_line_matches(line: &str, term_id: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("id:") {
        return false;
    }
    obo_field_token(trimmed["id:".len()..].trim_start()) == Some(term_id)
}

fn set_single_line(
    text: &mut String,
    term_id: &str,
    prefix: &str,
    value: &str,
    _allow_multiple: bool,
) -> Result<()> {
    let nl = detect_newline(text);
    let line = format!("{prefix} {value}");
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    if let Some(idx) = block.lines().position(|l| l.trim_start().starts_with(prefix)) {
        let lines: Vec<&str> = block.lines().collect();
        let mut out = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == idx {
                out.push_str(&line);
            } else {
                out.push_str(l);
            }
            out.push_str(nl);
        }
        text.replace_range(start..end, &out);
        Ok(())
    } else {
        add_line_in_term(text, term_id, &line)
    }
}

fn set_or_add_line(text: &mut String, term_id: &str, prefix: &str, full_line: &str) -> Result<()> {
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    if block.lines().any(|l| l.trim_start().starts_with(prefix)) {
        remove_lines_with_prefix(text, term_id, prefix)?;
    }
    add_line_in_term(text, term_id, full_line)
}

fn add_line_in_term(text: &mut String, term_id: &str, line: &str) -> Result<()> {
    let nl = detect_newline(text);
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    let mut new_block = block.trim_end_matches(['\n', '\r']).to_string();
    if !new_block.is_empty() {
        new_block.push_str(nl);
    }
    new_block.push_str(line);
    new_block.push_str(nl);
    text.replace_range(start..end, &new_block);
    Ok(())
}

fn remove_lines_where(
    text: &mut String,
    term_id: &str,
    should_remove: impl Fn(&str) -> bool,
    not_found: Option<&str>,
) -> Result<()> {
    let nl = detect_newline(text);
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    let lines: Vec<&str> = block.lines().collect();
    let removed_any = lines.iter().any(|l| should_remove(l));
    if let Some(message) = not_found {
        if !removed_any {
            return Err(OboError::PatchInvalid(message.to_string()));
        }
    }
    let kept: Vec<&str> = lines.into_iter().filter(|l| !should_remove(l)).collect();
    let mut new_block = kept.join(nl);
    if !new_block.is_empty() {
        new_block.push_str(nl);
    }
    text.replace_range(start..end, &new_block);
    Ok(())
}

fn remove_lines_with_prefix(text: &mut String, term_id: &str, prefix: &str) -> Result<()> {
    remove_lines_where(text, term_id, |l| l.trim_start().starts_with(prefix), None)
}

fn remove_is_a_line(text: &mut String, term_id: &str, parent_id: &str) -> Result<()> {
    let parent_id = parent_id.to_string();
    let not_found = format!("is_a parent not found: {parent_id}");
    remove_lines_where(text, term_id, move |l| is_is_a_parent_line(l, &parent_id), Some(&not_found))
}

fn remove_xref_line(text: &mut String, term_id: &str, xref: &str) -> Result<()> {
    let xref = xref.to_string();
    let not_found = format!("xref not found: {xref}");
    remove_lines_where(text, term_id, move |l| is_xref_line(l, &xref), Some(&not_found))
}

fn remove_synonym_line(
    text: &mut String,
    term_id: &str,
    value: &str,
    scope: Option<&str>,
) -> Result<()> {
    let (start, end) = term_block_range(text, term_id)?;
    let block = &text[start..end];
    let matches: Vec<&str> =
        block.lines().filter(|l| is_synonym_match_line(l, value, scope)).collect();
    if matches.is_empty() {
        return Err(OboError::PatchInvalid(format!("synonym not found: {value}")));
    }
    if scope.is_none() && matches.len() > 1 {
        return Err(OboError::PatchInvalid(format!(
            "multiple synonyms with text {value:?}; specify scope to disambiguate"
        )));
    }
    let value = value.to_string();
    let scope = scope.map(str::to_string);
    let not_found = format!("synonym not found: {value}");
    remove_lines_where(
        text,
        term_id,
        move |l| is_synonym_match_line(l, &value, scope.as_deref()),
        Some(&not_found),
    )
}

fn is_synonym_match_line(line: &str, value: &str, scope: Option<&str>) -> bool {
    let Some((v, line_scope)) = parse_synonym_value_and_scope(line) else {
        return false;
    };
    if v != value {
        return false;
    }
    match scope {
        Some(want) => line_scope.eq_ignore_ascii_case(want),
        None => true,
    }
}

fn parse_synonym_value_and_scope(line: &str) -> Option<(String, String)> {
    let value = parse_quoted_value_after_prefix(line, "synonym:")?;
    let trimmed = line.trim_start();
    let after_prefix = trimmed.strip_prefix("synonym:")?.trim_start();
    if !after_prefix.starts_with('"') {
        return None;
    }
    // Walk past the quoted value (same unescape rules as parse_quoted_value_after_prefix).
    let mut rest = &after_prefix[1..];
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            let _ = chars.next()?;
        } else if c == '"' {
            rest = chars.as_str().trim_start();
            break;
        }
    }
    let scope_end =
        rest.find(|c: char| c.is_whitespace() || c == '[' || c == '{').unwrap_or(rest.len());
    let scope = rest[..scope_end].trim();
    if scope.is_empty() {
        return None;
    }
    Some((value, scope.to_string()))
}

fn obo_field_token(rest: &str) -> Option<&str> {
    let end = rest.find(|c: char| c.is_whitespace() || c == '!' || c == '{').unwrap_or(rest.len());
    let token = rest[..end].trim();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

fn is_is_a_parent_line(line: &str, parent_id: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("is_a:") {
        return false;
    }
    obo_field_token(trimmed["is_a:".len()..].trim_start()) == Some(parent_id)
}

fn is_xref_line(line: &str, xref: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("xref:") {
        return false;
    }
    obo_field_token(trimmed["xref:".len()..].trim_start()) == Some(xref)
}

fn parse_quoted_value_after_prefix(line: &str, prefix: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with(prefix) {
        return None;
    }
    let mut rest = trimmed[prefix.len()..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    rest = &rest[1..];
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            out.push(chars.next()?);
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    None
}

/// Atomically replace `path` with `contents`, preserving existing permission bits (#422).
pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    strixonomy_core::atomic_write(path, contents).map_err(|e| OboError::Io(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: example
is_a: EX:000 ! root

[Term]
id: EX:002
name: other
"#;

    #[test]
    fn set_name_and_add_synonym() {
        let result = apply_patches_to_text(
            SAMPLE,
            &[
                OboPatchOp::SetName { term_id: "EX:001".into(), value: "renamed".into() },
                OboPatchOp::AddSynonym {
                    term_id: "EX:001".into(),
                    value: "alias".into(),
                    scope: "EXACT".into(),
                },
            ],
            true,
        )
        .expect("patch");
        let text = result.preview_text.expect("preview");
        assert!(text.contains("name: renamed"));
        assert!(text.contains("synonym: \"alias\" EXACT"));
    }

    #[test]
    fn add_and_remove_is_a() {
        let added = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::AddIsA { term_id: "EX:002".into(), parent_id: "EX:001".into() }],
            true,
        )
        .expect("add");
        let with_parent = added.preview_text.expect("preview");
        assert!(with_parent.contains("is_a: EX:001"));

        let removed = apply_patches_to_text(
            &with_parent,
            &[OboPatchOp::RemoveIsA { term_id: "EX:002".into(), parent_id: "EX:001".into() }],
            true,
        )
        .expect("remove");
        let text = removed.preview_text.expect("preview");
        let (start, end) = term_block_range(&text, "EX:002").expect("term block");
        assert!(
            !text[start..end].lines().any(|l| is_is_a_parent_line(l, "EX:001")),
            "EX:001 parent must be removed from EX:002"
        );
    }

    #[test]
    fn remove_is_a_does_not_match_prefix_collision_parent_ids() {
        const COLLISION: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:002
name: child
is_a: EX:0010 ! longer parent
is_a: EX:001 ! shorter parent
"#;

        let result = apply_patches_to_text(
            COLLISION,
            &[OboPatchOp::RemoveIsA { term_id: "EX:002".into(), parent_id: "EX:001".into() }],
            true,
        )
        .expect("remove shorter parent only");

        let text = result.preview_text.expect("preview");
        assert!(
            text.lines().any(|l| l.contains("is_a: EX:0010")),
            "EX:0010 parent must remain when removing EX:001"
        );
        assert!(
            !text.lines().any(|l| is_is_a_parent_line(l, "EX:001")),
            "EX:001 parent must be removed"
        );
    }

    #[test]
    fn set_name_targets_exact_term_id_not_prefix() {
        const COLLISION: &str = r#"format-version: 1.2
ontology: test

[Term]
id: GO:00000010
name: ten

[Term]
id: GO:0000001
name: one
"#;

        let result = apply_patches_to_text(
            COLLISION,
            &[OboPatchOp::SetName { term_id: "GO:0000001".into(), value: "one renamed".into() }],
            true,
        )
        .expect("patch shorter id");

        let text = result.preview_text.expect("preview");
        let (short_start, short_end) = term_block_range(&text, "GO:0000001").expect("short term");
        let (long_start, long_end) = term_block_range(&text, "GO:00000010").expect("long term");
        assert!(text[short_start..short_end].contains("name: one renamed"));
        assert!(text[long_start..long_end].contains("name: ten"));
        assert!(!text[long_start..long_end].contains("one renamed"));
    }

    #[test]
    fn term_block_range_stops_before_typedef_stanza() {
        const MIXED: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Typedef]
id: EX:rel
name: related

[Term]
id: EX:002
name: B
"#;

        let (start, end) = term_block_range(MIXED, "EX:001").expect("term block");
        let block = &MIXED[start..end];
        assert!(block.contains("id: EX:001"));
        assert!(block.contains("name: A"));
        assert!(
            !block.contains("[Typedef]"),
            "Typedef must not be part of the preceding Term block: {block}"
        );
        assert!(!block.contains("EX:rel"));
        assert!(!block.contains("id: EX:002"));
    }

    #[test]
    fn set_name_preserves_intervening_typedef() {
        const MIXED: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Typedef]
id: EX:rel
name: related

[Term]
id: EX:002
name: B
"#;

        let result = apply_patches_to_text(
            MIXED,
            &[OboPatchOp::SetName { term_id: "EX:001".into(), value: "A renamed".into() }],
            true,
        )
        .expect("set name");
        let text = result.preview_text.expect("preview");
        assert!(text.contains("name: A renamed"));
        assert!(
            text.contains("[Typedef]\nid: EX:rel\nname: related"),
            "Typedef stanza must survive term edit: {text}"
        );
        assert!(text.contains("id: EX:002\nname: B"));
    }

    #[test]
    fn add_synonym_preserves_intervening_instance_stanza() {
        const MIXED: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Instance]
id: EX:inst
name: sample

[Term]
id: EX:002
name: B
"#;

        let result = apply_patches_to_text(
            MIXED,
            &[OboPatchOp::AddSynonym {
                term_id: "EX:001".into(),
                value: "alias".into(),
                scope: "EXACT".into(),
            }],
            true,
        )
        .expect("add synonym");
        let text = result.preview_text.expect("preview");
        assert!(text.contains("synonym: \"alias\" EXACT"));
        assert!(
            text.contains("[Instance]\nid: EX:inst\nname: sample"),
            "Instance stanza must survive term edit: {text}"
        );
    }

    #[test]
    fn remove_is_a_errors_when_parent_missing() {
        let err = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::RemoveIsA { term_id: "EX:001".into(), parent_id: "EX:999".into() }],
            true,
        )
        .expect("patch result");
        assert!(!err.diagnostics.is_empty());
        assert!(err.diagnostics[0].message.contains("is_a parent not found"));
    }

    #[test]
    fn rejects_set_name_with_embedded_newline() {
        let result = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::SetName {
                term_id: "EX:001".into(),
                value: "evil\nis_a: ATTACK:001".into(),
            }],
            true,
        )
        .expect("patch result");
        assert!(!result.applied);
        assert!(result.diagnostics.iter().any(|d| d.message.contains("line breaks")));
        assert_eq!(result.preview_text.as_deref(), Some(SAMPLE));
    }

    #[test]
    fn rejects_add_xref_with_embedded_newline() {
        let result = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::AddXref {
                term_id: "EX:001".into(),
                xref: "FOO:1\nname: injected".into(),
            }],
            true,
        )
        .expect("patch result");
        assert!(!result.applied);
        assert!(result.diagnostics.iter().any(|d| d.message.contains("line breaks")));
    }

    #[test]
    fn rejects_add_is_a_with_whitespace_in_parent_id() {
        let result = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::AddIsA { term_id: "EX:002".into(), parent_id: "EX:001 injected".into() }],
            true,
        )
        .expect("patch result");
        assert!(!result.applied);
        assert!(result.diagnostics.iter().any(|d| d.message.contains("whitespace")));
    }

    #[test]
    fn atomic_write_overwrites_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("term.obo");
        std::fs::write(&path, "format-version: 1.2\nold\n").unwrap();
        atomic_write(&path, "format-version: 1.2\nontology: new\n").expect("atomic write");
        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains("ontology: new"));
        assert!(!contents.contains("old"));
        let leftovers: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "temp files left behind: {leftovers:?}");
    }

    #[cfg(unix)]
    #[test]
    fn atomic_write_preserves_unix_mode() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("secret.obo");
        std::fs::write(&path, "format-version: 1.2\nold\n").unwrap();
        let mut perms = std::fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&path, perms).unwrap();
        atomic_write(&path, "format-version: 1.2\nontology: private\n").expect("atomic write");
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600,
            "mode must survive OBO atomic write (#422)"
        );
    }

    #[test]
    fn set_name_on_typedef_does_not_rewrite_preceding_term() {
        const MIXED: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:001
name: A

[Typedef]
id: EX:rel
name: related
"#;

        let result = apply_patches_to_text(
            MIXED,
            &[OboPatchOp::SetName { term_id: "EX:rel".into(), value: "renamed".into() }],
            true,
        )
        .expect("set typedef name");
        let text = result.preview_text.expect("preview");
        let (term_start, term_end) = term_block_range(&text, "EX:001").expect("term");
        let (td_start, td_end) = term_block_range(&text, "EX:rel").expect("typedef");
        assert!(text[term_start..term_end].contains("name: A"));
        assert!(!text[term_start..term_end].contains("renamed"));
        assert!(text[td_start..td_end].contains("name: renamed"));
        assert!(!text[td_start..td_end].contains("name: related"));
    }

    #[test]
    fn remove_synonym_requires_scope_when_text_is_ambiguous() {
        const MULTI: &str = r#"format-version: 1.2
ontology: ex

[Term]
id: EX:1
name: t
synonym: "foo" EXACT []
synonym: "foo" BROAD []
"#;

        let ambiguous = apply_patches_to_text(
            MULTI,
            &[OboPatchOp::RemoveSynonym {
                term_id: "EX:1".into(),
                value: "foo".into(),
                scope: None,
            }],
            true,
        )
        .expect("patch result");
        assert!(!ambiguous.applied);
        assert!(ambiguous.diagnostics.iter().any(|d| d.message.contains("multiple synonyms")));

        let scoped = apply_patches_to_text(
            MULTI,
            &[OboPatchOp::RemoveSynonym {
                term_id: "EX:1".into(),
                value: "foo".into(),
                scope: Some("EXACT".into()),
            }],
            true,
        )
        .expect("scoped remove");
        let text = scoped.preview_text.expect("preview");
        assert!(!text.contains(r#"synonym: "foo" EXACT"#));
        assert!(text.contains(r#"synonym: "foo" BROAD"#));
    }

    #[test]
    fn set_name_preserves_crlf_line_endings() {
        let crlf = SAMPLE.replace('\n', "\r\n");
        let result = apply_patches_to_text(
            &crlf,
            &[OboPatchOp::SetName { term_id: "EX:001".into(), value: "renamed".into() }],
            true,
        )
        .expect("patch");
        let text = result.preview_text.expect("preview");
        assert!(text.contains("name: renamed\r\n"));
        assert!(text.contains("\r\n"), "must keep CRLF");
        assert!(!text.replace("\r\n", "").contains('\r'));
        // Rewritten region should not introduce bare LF joins.
        assert!(
            !text.contains("renamed\nname:") && text.contains("renamed\r\n"),
            "name line must end with CRLF"
        );
    }

    #[test]
    fn add_synonym_escapes_embedded_quotes() {
        let result = apply_patches_to_text(
            SAMPLE,
            &[OboPatchOp::AddSynonym {
                term_id: "EX:001".into(),
                value: r#"foo "bar""#.into(),
                scope: "EXACT".into(),
            }],
            true,
        )
        .expect("add synonym");
        let text = result.preview_text.expect("preview");
        assert!(text.contains(r#"synonym: "foo \"bar\"" EXACT []"#));
    }
}
