const UPDATE_VERBS: &[&str] =
    &["INSERT", "DELETE", "LOAD", "CLEAR", "CREATE", "DROP", "MOVE", "COPY", "ADD"];

/// Returns true when the query appears to be a SPARQL update (not read-only).
pub fn is_sparql_update(sparql: &str) -> bool {
    let mut rest = sparql;
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            return false;
        }
        if let Some(after_hash) = rest.strip_prefix('#') {
            rest = after_hash.split_once('\n').map_or("", |(_, tail)| tail);
            continue;
        }
        // Skip /* block comments */
        if rest.starts_with("/*") {
            rest = rest.split_once("*/").map_or("", |(_, tail)| tail);
            continue;
        }
        let upper = rest.to_ascii_uppercase();
        // Skip prologue declarations (PREFIX / BASE), including same-line forms.
        if upper.starts_with("PREFIX") || upper.starts_with("BASE") {
            rest = skip_sparql_prologue_decl(rest);
            continue;
        }
        // Skip WITH <graph> so WITH … INSERT/DELETE is still classified as an update.
        if upper.starts_with("WITH")
            && upper.as_bytes().get(4).is_none_or(|b| b.is_ascii_whitespace() || *b == b'<')
        {
            rest = skip_with_clause(rest);
            continue;
        }
        // Skip USING / USING NAMED <graph> (SPARQL Update dataset clause; #399).
        if upper.starts_with("USING")
            && upper.as_bytes().get(5).is_none_or(|b| b.is_ascii_whitespace())
        {
            rest = skip_using_clause(rest);
            continue;
        }
        return UPDATE_VERBS.iter().any(|verb| {
            upper.starts_with(verb)
                && upper
                    .as_bytes()
                    .get(verb.len())
                    .is_none_or(|b| b.is_ascii_whitespace() || *b == b'{')
        });
    }
}

/// Advance past a PREFIX or BASE declaration (line- or token-oriented).
fn skip_sparql_prologue_decl(rest: &str) -> &str {
    let upper = rest.to_ascii_uppercase();
    if upper.starts_with("PREFIX") {
        // PREFIX name: <iri>  — skip through the closing '>' if present, else end of line.
        if let Some(gt) = rest.find('>') {
            return rest[gt + 1..].trim_start();
        }
        return rest.split_once('\n').map_or("", |(_, tail)| tail);
    }
    if upper.starts_with("BASE") {
        if let Some(gt) = rest.find('>') {
            return rest[gt + 1..].trim_start();
        }
        return rest.split_once('\n').map_or("", |(_, tail)| tail);
    }
    rest
}

/// Advance past `WITH <graph>` (optionally followed by more prologue before the update verb).
fn skip_with_clause(rest: &str) -> &str {
    if let Some(gt) = rest.find('>') {
        return rest[gt + 1..].trim_start();
    }
    rest.split_once('\n').map_or("", |(_, tail)| tail)
}

/// Advance past `USING [NAMED] <graph>`.
fn skip_using_clause(rest: &str) -> &str {
    let upper = rest.to_ascii_uppercase();
    let after_using = rest.get(5..).unwrap_or("").trim_start();
    let after_using_upper = upper.get(5..).unwrap_or("").trim_start();
    let body = if after_using_upper.starts_with("NAMED")
        && after_using_upper.as_bytes().get(5).is_none_or(|b| b.is_ascii_whitespace() || *b == b'<')
    {
        after_using.get(5..).unwrap_or("").trim_start()
    } else {
        after_using
    };
    if let Some(gt) = body.find('>') {
        // `body` is a suffix of `rest`; map the '>' offset back.
        let prefix_len = rest.len() - body.len();
        return rest[prefix_len + gt + 1..].trim_start();
    }
    rest.split_once('\n').map_or("", |(_, tail)| tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_insert_after_prefix() {
        let q = "PREFIX ex: <http://example.org/>\nINSERT DATA { ex:s ex:p ex:o }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn allows_select_after_prefix() {
        let q = "PREFIX ex: <http://example.org/>\nSELECT ?s WHERE { ?s ?p ?o }";
        assert!(!is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_after_comment() {
        let q = "# setup\nINSERT DATA { }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_after_base() {
        let q = "BASE <http://example.org/>\nINSERT DATA { <s> <p> <o> }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_on_same_line_as_prefix() {
        let q = "PREFIX ex: <http://example.org/> INSERT DATA { ex:s ex:p ex:o }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn allows_select_after_base() {
        let q = "BASE <http://example.org/>\nSELECT ?s WHERE { ?s ?p ?o }";
        assert!(!is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_after_with() {
        let q = "WITH <http://example.org/graph>\nINSERT DATA { <http://ex/s> <http://ex/p> <http://ex/o> }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_after_using() {
        let q = "USING <http://example.org/g>\nINSERT DATA { <http://ex/s> <http://ex/p> <http://ex/o> }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_after_using_named() {
        let q = "USING NAMED <http://example.org/g>\nINSERT DATA { <http://ex/s> <http://ex/p> <http://ex/o> }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_delete_after_prefix_and_using() {
        let q = "PREFIX ex: <http://example.org/>\nUSING <http://example.org/g>\nDELETE DATA { ex:s ex:p ex:o }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_delete_after_prefix_and_with() {
        let q = "PREFIX ex: <http://example.org/>\nWITH <http://example.org/g>\nDELETE DATA { ex:s ex:p ex:o }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn allows_select_when_with_is_not_prologue() {
        // "WITHIN" must not be treated as a WITH clause.
        let q = "SELECT ?within WHERE { ?s ?p ?within }";
        assert!(!is_sparql_update(q));
    }

    #[test]
    fn allows_select_when_using_is_not_prologue() {
        // Variable/name starting with USING… must not trip the detector.
        let q = "SELECT ?usingNamed WHERE { ?s ?p ?usingNamed }";
        assert!(!is_sparql_update(q));
    }
}
