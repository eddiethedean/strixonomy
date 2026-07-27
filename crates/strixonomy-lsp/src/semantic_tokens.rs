//! Lightweight semantic tokenization for Turtle and OBO sources.

use lsp_types::{
    SemanticToken, SemanticTokens, SemanticTokensLegend, SemanticTokensParams, SemanticTokensResult,
};

const TOKEN_NAMESPACE: u32 = 0;
const TOKEN_IRI: u32 = 1;
const TOKEN_KEYWORD: u32 = 2;
const TOKEN_COMMENT: u32 = 3;
const TOKEN_STRING: u32 = 4;

pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            "namespace".into(),
            "iri".into(),
            "keyword".into(),
            "comment".into(),
            "string".into(),
        ],
        token_modifiers: vec![],
    }
}

pub fn handle_semantic_tokens_full(
    params: SemanticTokensParams,
    doc_text: Option<String>,
) -> Option<SemanticTokensResult> {
    let uri = params.text_document.uri.as_str();
    let text = doc_text?;
    let is_obo = uri.ends_with(".obo");
    let tokens = if is_obo { tokenize_obo(&text) } else { tokenize_turtle(&text) };
    Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data: tokens }))
}

/// LSP semantic-token lengths / columns are UTF-16 code units (#403).
fn utf16_len(s: &str) -> u32 {
    s.chars().map(|c| c.len_utf16() as u32).sum()
}

fn tokenize_turtle(text: &str) -> Vec<SemanticToken> {
    let mut out = Vec::new();
    let mut line = 0u32;
    let mut col = 0u32;
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            line += 1;
            col = 0;
            i += 1;
            continue;
        }
        if b == b'#' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            let len = utf16_len(&text[start..i]);
            push_token(&mut out, line, col, len, TOKEN_COMMENT);
            col += len;
            continue;
        }
        if b == b'"' || b == b'\'' {
            let byte_len = scan_string(&bytes[i..]);
            let len = utf16_len(&text[i..i + byte_len]);
            push_token(&mut out, line, col, len, TOKEN_STRING);
            i += byte_len;
            col += len;
            continue;
        }
        if b == b'<' {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i] != b'>' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            let len = utf16_len(&text[start..i]);
            push_token(&mut out, line, col, len, TOKEN_IRI);
            col += len;
            continue;
        }
        if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let word = &text[start..i];
            let kind = if TURTLE_KEYWORDS.contains(&word) {
                TOKEN_KEYWORD
            } else if word.contains(':') {
                TOKEN_NAMESPACE
            } else {
                TOKEN_KEYWORD
            };
            let len = utf16_len(word);
            push_token(&mut out, line, col, len, kind);
            col += len;
            continue;
        }
        // Advance one Unicode scalar (not one UTF-8 byte) so columns stay UTF-16-correct.
        let ch = text[i..].chars().next().expect("non-empty slice");
        i += ch.len_utf8();
        col += ch.len_utf16() as u32;
    }
    out
}

fn tokenize_obo(text: &str) -> Vec<SemanticToken> {
    let mut out = Vec::new();
    let mut line = 0u32;
    let mut col = 0u32;
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            line += 1;
            col = 0;
            i += 1;
            continue;
        }
        if b == b'!' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            let len = utf16_len(&text[start..i]);
            push_token(&mut out, line, col, len, TOKEN_COMMENT);
            col += len;
            continue;
        }
        if b == b'"' {
            let byte_len = scan_string(&bytes[i..]);
            let len = utf16_len(&text[i..i + byte_len]);
            push_token(&mut out, line, col, len, TOKEN_STRING);
            i += byte_len;
            col += len;
            continue;
        }
        if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) && bytes[i] != b':' {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b':' {
                i += 1;
                while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'\t' && bytes[i] != b'\n'
                {
                    i += 1;
                }
                let len = utf16_len(&text[start..i]);
                push_token(&mut out, line, col, len, TOKEN_KEYWORD);
                col += len;
            } else {
                let len = utf16_len(&text[start..i]);
                push_token(&mut out, line, col, len, TOKEN_NAMESPACE);
                col += len;
            }
            continue;
        }
        let ch = text[i..].chars().next().expect("non-empty slice");
        i += ch.len_utf8();
        col += ch.len_utf16() as u32;
    }
    out
}

const TURTLE_KEYWORDS: &[&str] = &["@prefix", "@base", "a", "true", "false", "PREFIX", "BASE"];

fn scan_string(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        return 0;
    }
    let quote = bytes[0];
    let mut i = 1;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == quote {
            i += 1;
            break;
        }
        i += 1;
    }
    i
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'@'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b':'
}

fn push_token(out: &mut Vec<SemanticToken>, line: u32, start_char: u32, length: u32, kind: u32) {
    if length == 0 {
        return;
    }
    let (prev_line, prev_start) = absolute_position_of_last(out);
    let delta_line = line.saturating_sub(prev_line);
    let delta_start = if out.is_empty() || delta_line > 0 {
        start_char
    } else {
        start_char.saturating_sub(prev_start)
    };
    out.push(SemanticToken {
        delta_line,
        delta_start,
        length,
        token_type: kind,
        token_modifiers_bitset: 0,
    });
}

/// Reconstruct the absolute `(line, start_char)` of the last encoded token.
fn absolute_position_of_last(tokens: &[SemanticToken]) -> (u32, u32) {
    let mut line = 0u32;
    let mut start = 0u32;
    for token in tokens {
        if token.delta_line > 0 {
            line += token.delta_line;
            start = token.delta_start;
        } else {
            start += token.delta_start;
        }
    }
    (line, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Decode LSP semantic-token deltas back to absolute `(line, start_char, length, type)`.
    fn decode_absolute_positions(tokens: &[SemanticToken]) -> Vec<(u32, u32, u32, u32)> {
        let mut line = 0u32;
        let mut start = 0u32;
        let mut out = Vec::with_capacity(tokens.len());
        for token in tokens {
            if token.delta_line > 0 {
                line += token.delta_line;
                start = token.delta_start;
            } else if out.is_empty() {
                start = token.delta_start;
            } else {
                start += token.delta_start;
            }
            out.push((line, start, token.length, token.token_type));
        }
        out
    }

    #[test]
    fn turtle_tokens_include_comment_and_prefix() {
        let text = "@prefix ex: <http://ex/> .\n# comment\nex:s a ex:C .";
        let tokens = tokenize_turtle(text);
        assert!(!tokens.is_empty());
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_COMMENT));
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_NAMESPACE));
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_KEYWORD));
    }

    #[test]
    fn semantic_token_deltas_decode_to_absolute_spans() {
        let text = "@prefix ex: <http://ex/> .\n\nex:A a ex:B .\n\n# trailing";
        let tokens = tokenize_turtle(text);
        let absolute = decode_absolute_positions(&tokens);

        // First token is `@prefix` on line 0.
        assert_eq!(absolute[0].0, 0);
        assert_eq!(absolute[0].3, TOKEN_KEYWORD);

        // Tokens after the blank line must stay on line 2, not drift forward.
        let line2: Vec<_> = absolute.iter().filter(|t| t.0 == 2).collect();
        assert!(
            !line2.is_empty(),
            "expected tokens on line 2, got absolute={absolute:?} deltas={tokens:?}"
        );
        assert!(
            absolute.iter().any(|t| t.0 == 2 && t.3 == TOKEN_NAMESPACE),
            "expected namespace token on line 2: {absolute:?}"
        );
        assert!(
            absolute.iter().any(|t| t.0 == 4 && t.3 == TOKEN_COMMENT),
            "expected comment on line 4: {absolute:?}"
        );
        assert!(
            absolute.iter().all(|t| t.0 <= 4),
            "decoded lines drifted past document end: {absolute:?}"
        );
    }

    #[test]
    fn push_token_deltas_use_absolute_previous_position() {
        let mut out = Vec::new();
        push_token(&mut out, 0, 0, 7, TOKEN_KEYWORD); // line 0
        push_token(&mut out, 2, 0, 3, TOKEN_NAMESPACE); // line 2
        push_token(&mut out, 2, 4, 1, TOKEN_KEYWORD); // same line
        push_token(&mut out, 5, 0, 1, TOKEN_COMMENT); // line 5

        assert_eq!(
            decode_absolute_positions(&out),
            vec![
                (0, 0, 7, TOKEN_KEYWORD),
                (2, 0, 3, TOKEN_NAMESPACE),
                (2, 4, 1, TOKEN_KEYWORD),
                (5, 0, 1, TOKEN_COMMENT),
            ]
        );
    }

    #[test]
    fn turtle_tokens_mark_iris_in_angle_brackets() {
        let text = "@prefix ex: <http://example.org/> .\nex:A a <http://example.org/B> .";
        let tokens = tokenize_turtle(text);
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_IRI));
    }

    #[test]
    fn turtle_string_token_length_is_utf16_not_utf8_bytes() {
        // #403 — CJK label must report UTF-16 length (3 chars → 3 units), not UTF-8 byte count.
        let text = r#"ex:C rdfs:label "日本語" ."#;
        let tokens = tokenize_turtle(text);
        let absolute = decode_absolute_positions(&tokens);
        let string_tok = absolute.iter().find(|t| t.3 == TOKEN_STRING).expect("string token");
        // `"日本語"` = quote + 3 BMP chars + quote → 5 UTF-16 units (not 11 UTF-8 bytes).
        assert_eq!(string_tok.2, 5, "absolute={absolute:?}");
    }

    #[test]
    fn obo_tokens_mark_comments_and_tags() {
        let text = "format-version: 1.2\n! comment\nid: GO:0000001\nname: root\n";
        let tokens = tokenize_obo(text);
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_COMMENT));
        assert!(tokens.iter().any(|t| t.token_type == TOKEN_KEYWORD));
    }

    #[test]
    fn handle_semantic_tokens_full_requires_document_text() {
        use lsp_types::{SemanticTokensParams, TextDocumentIdentifier, Uri};
        use std::str::FromStr;

        let params = SemanticTokensParams {
            text_document: TextDocumentIdentifier {
                uri: Uri::from_str("file:///example.ttl").unwrap(),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        assert!(handle_semantic_tokens_full(params, None).is_none());
        let params = SemanticTokensParams {
            text_document: TextDocumentIdentifier {
                uri: Uri::from_str("file:///example.ttl").unwrap(),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let result = handle_semantic_tokens_full(params, Some("@prefix ex: <http://ex#> .".into()));
        assert!(result.is_some());
    }
}
