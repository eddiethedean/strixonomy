//! Canonical Turtle string/comment/IRI lexer used by span scanning and patch operations.
//!
//! Diagnostics maintain a parallel char-based stripper in `strixonomy-diagnostics`; keep rules aligned.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TurtleStringKind {
    ShortDouble,
    ShortSingle,
    LongDouble,
    LongSingle,
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct TurtleScanState {
    pub string_kind: Option<TurtleStringKind>,
    pub in_iri: bool,
    pub in_comment: bool,
    pub escape: bool,
}

impl TurtleScanState {
    pub fn in_string(&self) -> bool {
        self.string_kind.is_some()
    }

    pub fn in_comment_or_string(&self) -> bool {
        self.in_comment || self.string_kind.is_some()
    }
}

/// Advance the lexer by one token step from `i`, updating `state`. Returns the new index.
pub(crate) fn advance_turtle_scan(bytes: &[u8], i: usize, state: &mut TurtleScanState) -> usize {
    if i >= bytes.len() {
        return i;
    }

    if state.in_comment {
        if bytes[i] == b'\n' {
            state.in_comment = false;
        }
        return i + 1;
    }

    if let Some(kind) = state.string_kind {
        return advance_inside_string(bytes, i, state, kind);
    }

    if state.in_iri {
        if bytes[i] == b'>' {
            state.in_iri = false;
        }
        return i + 1;
    }

    if bytes.get(i..i + 3) == Some(br#"""""#) {
        state.string_kind = Some(TurtleStringKind::LongDouble);
        return i + 3;
    }
    if bytes.get(i..i + 3) == Some(br"'''") {
        state.string_kind = Some(TurtleStringKind::LongSingle);
        return i + 3;
    }

    match bytes[i] {
        b'#' => {
            state.in_comment = true;
            i + 1
        }
        b'"' => {
            state.string_kind = Some(TurtleStringKind::ShortDouble);
            i + 1
        }
        b'\'' => {
            state.string_kind = Some(TurtleStringKind::ShortSingle);
            i + 1
        }
        b'<' => {
            state.in_iri = true;
            i + 1
        }
        _ => i + 1,
    }
}

fn advance_inside_string(
    bytes: &[u8],
    i: usize,
    state: &mut TurtleScanState,
    kind: TurtleStringKind,
) -> usize {
    let b = bytes[i];
    match kind {
        TurtleStringKind::ShortDouble | TurtleStringKind::ShortSingle => {
            let quote = if kind == TurtleStringKind::ShortDouble { b'"' } else { b'\'' };
            if state.escape {
                state.escape = false;
            } else if b == b'\\' {
                state.escape = true;
            } else if b == quote {
                state.string_kind = None;
            }
            i + 1
        }
        TurtleStringKind::LongDouble => {
            if state.escape {
                state.escape = false;
                i + 1
            } else if b == b'\\' {
                state.escape = true;
                i + 1
            } else if bytes.get(i..i + 3) == Some(br#"""""#) {
                state.string_kind = None;
                i + 3
            } else {
                i + 1
            }
        }
        TurtleStringKind::LongSingle => {
            if state.escape {
                state.escape = false;
                i + 1
            } else if b == b'\\' {
                state.escape = true;
                i + 1
            } else if bytes.get(i..i + 3) == Some(br"'''") {
                state.string_kind = None;
                i + 3
            } else {
                i + 1
            }
        }
    }
}

/// True when `byte_offset` lies inside a `#` line comment or any Turtle string literal.
pub fn is_in_comment_or_string_at(text: &str, byte_offset: usize) -> bool {
    let bytes = text.as_bytes();
    let mut state = TurtleScanState::default();
    let mut i = 0usize;
    while i < byte_offset && i < bytes.len() {
        i = advance_turtle_scan(bytes, i, &mut state);
    }
    state.in_comment_or_string()
}

/// Quoted span of a Turtle literal object, ignoring trailing `@lang` / `^^datatype`.
fn turtle_quoted_literal_span(trimmed: &str) -> Option<&str> {
    let bytes = trimmed.as_bytes();
    if !matches!(bytes.first(), Some(b'"' | b'\'')) {
        return None;
    }
    let mut state = TurtleScanState::default();
    let mut i = 0usize;
    i = advance_turtle_scan(bytes, i, &mut state);
    if !state.in_string() {
        return None;
    }
    while i < bytes.len() && state.in_string() {
        i = advance_turtle_scan(bytes, i, &mut state);
    }
    if state.in_string() || i == 0 {
        return None;
    }
    Some(&trimmed[..i])
}

/// Decode a Turtle string literal (`"…"`, `'…'`, `"""…"""`, `'''…'''`) to its lexical value.
///
/// Language tags (`"Person"@en`) and datatypes (`"42"^^xsd:integer`) are stripped so
/// remove-by-lexical-value matches the quoted form.
pub(crate) fn turtle_literal_lexical_value(literal: &str) -> Option<String> {
    let trimmed = literal.trim();
    let quoted = turtle_quoted_literal_span(trimmed).unwrap_or(trimmed);
    if quoted.len() < 2 {
        return None;
    }
    if quoted.starts_with("\"\"\"") && quoted.ends_with("\"\"\"") && quoted.len() >= 6 {
        return Some(unescape_turtle_string(&quoted[3..quoted.len() - 3]));
    }
    if quoted.starts_with("'''") && quoted.ends_with("'''") && quoted.len() >= 6 {
        return Some(unescape_turtle_string(&quoted[3..quoted.len() - 3]));
    }
    if quoted.starts_with('"') && quoted.ends_with('"') {
        return Some(unescape_turtle_string(&quoted[1..quoted.len() - 1]));
    }
    if quoted.starts_with('\'') && quoted.ends_with('\'') {
        return Some(unescape_turtle_string(&quoted[1..quoted.len() - 1]));
    }
    None
}

/// True when `text` contains `token` as a Turtle name token outside comments, strings, and IRIs.
pub(crate) fn has_turtle_name_token(text: &str, token: &str) -> bool {
    let bytes = text.as_bytes();
    let needle = token.as_bytes();
    if needle.is_empty() {
        return false;
    }
    let mut i = 0usize;
    let mut state = TurtleScanState::default();
    while i + needle.len() <= bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if matches!(bytes.get(i), Some(b'#' | b'"' | b'\'' | b'<'))
            || bytes.get(i..i + 3) == Some(br#"""""#)
            || bytes.get(i..i + 3) == Some(br"'''")
        {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if bytes[i..].starts_with(needle) {
            let after = i + needle.len();
            let before_ok = i == 0
                || (!bytes[i - 1].is_ascii_alphanumeric()
                    && bytes[i - 1] != b':'
                    && bytes[i - 1] != b'_'
                    && bytes[i - 1] != b'-'
                    && bytes[i - 1] != b'.');
            let after_ok = after >= bytes.len()
                || bytes[after].is_ascii_whitespace()
                || matches!(bytes[after], b';' | b'.' | b',' | b')' | b']');
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// True when a Turtle statement declares `rdf:type` / shorthand `a` as a real predicate.
pub(crate) fn statement_has_type_predicate(stmt: &str) -> bool {
    has_turtle_name_token(stmt, "a") || has_turtle_name_token(stmt, "rdf:type")
}

fn unescape_turtle_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some('"') => out.push('"'),
                Some('\'') => out.push('\''),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if hex.len() == 4 {
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(decoded) = char::from_u32(code) {
                                out.push(decoded);
                                continue;
                            }
                        }
                    }
                    out.push('\\');
                    out.push('u');
                    out.push_str(&hex);
                }
                Some('U') => {
                    let hex: String = chars.by_ref().take(8).collect();
                    if hex.len() == 8 {
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(decoded) = char::from_u32(code) {
                                out.push(decoded);
                                continue;
                            }
                        }
                    }
                    out.push('\\');
                    out.push('U');
                    out.push_str(&hex);
                }
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_double_escape_does_not_close_early() {
        let text = r#"ex:A rdfs:comment """contains \""" marker""" ."#;
        let start = text.find("ex:A").unwrap();
        let end = crate::span::statement_end_byte(text, start).expect("end");
        assert!(end > text.find(r#"\""" marker"""#).unwrap());
        assert!(text[start..end].ends_with('.'));
    }

    #[test]
    fn long_single_escape_does_not_close_early() {
        let text = "ex:A rdfs:comment '''contains \\''' marker''' .";
        let start = text.find("ex:A").unwrap();
        let end = crate::span::statement_end_byte(text, start).expect("end");
        assert!(end > text.find("marker'''").unwrap());
    }

    #[test]
    fn lexical_value_all_quote_forms() {
        assert_eq!(turtle_literal_lexical_value(r#""Hello""#), Some("Hello".to_string()));
        assert_eq!(turtle_literal_lexical_value("'Hello'"), Some("Hello".to_string()));
        assert_eq!(turtle_literal_lexical_value(r#""""Hello""""#), Some("Hello".to_string()));
        assert_eq!(turtle_literal_lexical_value("'''Hello'''"), Some("Hello".to_string()));
    }

    #[test]
    fn lexical_value_strips_lang_and_datatype() {
        assert_eq!(turtle_literal_lexical_value(r#""Person"@en"#), Some("Person".to_string()));
        assert_eq!(turtle_literal_lexical_value(r#""42"^^xsd:integer"#), Some("42".to_string()));
        assert_eq!(
            turtle_literal_lexical_value(r#""x"^^<http://www.w3.org/2001/XMLSchema#string>"#),
            Some("x".to_string())
        );
    }

    #[test]
    fn lexical_value_unescapes() {
        assert_eq!(turtle_literal_lexical_value(r#""a\"b""#), Some(r#"a"b"#.to_string()));
        assert_eq!(turtle_literal_lexical_value("'a\\'b'"), Some("a'b".to_string()));
    }

    #[test]
    fn type_predicate_ignores_comment_substring() {
        let stmt = r#"ex:Foo rdfs:comment "was a prototype" ."#;
        assert!(!statement_has_type_predicate(stmt));
        let typed = r#"ex:Foo a owl:Class ."#;
        assert!(statement_has_type_predicate(typed));
        let rdf_type = r#"ex:Foo rdf:type owl:Class ."#;
        assert!(statement_has_type_predicate(rdf_type));
    }

    #[test]
    fn is_in_comment_or_string_long_forms() {
        let text = "rdfs:comment '''see rdfs:label''' .";
        let label_pos = text.find("rdfs:label").unwrap();
        assert!(is_in_comment_or_string_at(text, label_pos));
    }
}
