use std::collections::BTreeMap;
use strixonomy_core::document_lookup::normalize_iri;
use strixonomy_owl::{namespaces_for_text, short_name_from_iri};

pub fn normalize_namespace_base(base: &str) -> String {
    normalize_iri(base)
}

pub fn remap_iri(iri: &str, from_base: &str, to_base: &str) -> Option<String> {
    let from = normalize_namespace_base(from_base);
    let to = normalize_namespace_base(to_base);
    if iri == from || iri.starts_with(&format!("{from}#")) || iri.starts_with(&format!("{from}/")) {
        let suffix = if iri.len() > from.len() { &iri[from.len()..] } else { "" };
        Some(format!("{to}{suffix}"))
    } else {
        None
    }
}

/// Replace all occurrences of `old_iri` with `new_iri` in Turtle text, preserving prefix forms.
///
/// Skips matches inside `#` comments and Turtle string literals (`"…"`, `'…'`, `"""…"""`, `'''…'''`).
pub fn replace_iri_in_text(
    text: &str,
    old_iri: &str,
    new_iri: &str,
    declared_namespaces: &BTreeMap<String, String>,
) -> (String, Vec<(usize, usize, String, String)>) {
    let namespaces = namespaces_for_text(text, declared_namespaces);
    let old_short = short_name_from_iri(old_iri);
    let new_short = short_name_from_iri(new_iri);
    let mut hunks = Vec::new();
    let mut result = text.to_string();

    let mut replacements: Vec<(String, String)> =
        vec![(format!("<{old_iri}>"), format!("<{new_iri}>"))];
    // Prefer angle-bracket and prefixed forms; bare IRI only when not inside comments/strings
    // (enforced by `is_in_comment_or_string`).
    if old_iri != new_iri {
        replacements.push((old_iri.to_string(), new_iri.to_string()));
    }

    // Default-prefix `:LocalName` when the empty prefix binds to this IRI's namespace.
    if let Some(default_ns) = namespaces.get("") {
        if old_iri.starts_with(default_ns.as_str()) {
            let old_token = format!(":{old_short}");
            let new_token = format!(":{new_short}");
            if old_token != new_token {
                replacements.push((old_token, new_token));
            }
        }
    }

    for (prefix, ns) in &namespaces {
        if old_iri.starts_with(ns) && !prefix.is_empty() {
            let old_token = format!("{prefix}:{old_short}");
            let new_ns = namespace_for_iri(new_iri, &namespaces).unwrap_or_else(|| {
                new_iri
                    .rsplit_once('#')
                    .or_else(|| new_iri.rsplit_once('/'))
                    .map(|(base, _)| base.to_string())
                    .unwrap_or_default()
            });
            let new_token = match prefix_for_namespace(&new_ns, &namespaces) {
                Some(new_prefix) if new_prefix.is_empty() => format!(":{new_short}"),
                Some(new_prefix) => format!("{new_prefix}:{new_short}"),
                // Never reuse the old prefix for a different namespace — that would keep the
                // old IRI expansion. Fall back to an absolute IRI form.
                None => format!("<{new_iri}>"),
            };
            if old_token != new_token {
                replacements.push((old_token, new_token));
            }
        }
    }

    for (old, new) in &replacements {
        if old == new {
            continue;
        }
        let mut search_from = 0usize;
        while let Some(pos) = result[search_from..].find(old) {
            let start = search_from + pos;
            let end = start + old.len();
            if strixonomy_owl::is_in_comment_or_string(&result, start)
                || !is_safe_replacement_boundary(&result, start, end)
            {
                search_from = end;
                continue;
            }
            hunks.push((start, end, old.clone(), new.clone()));
            result.replace_range(start..end, new);
            search_from = start + new.len();
        }
    }

    (result, hunks)
}

fn namespace_for_iri(iri: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    let mut best: Option<(usize, String)> = None;
    for ns in namespaces.values() {
        if iri.starts_with(ns) {
            let len = ns.len();
            if best.as_ref().is_none_or(|(l, _)| len > *l) {
                best = Some((len, ns.clone()));
            }
        }
    }
    best.map(|(_, ns)| ns)
}

fn prefix_for_namespace(ns: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    namespaces.iter().find(|(_, v)| normalize_iri(v) == normalize_iri(ns)).map(|(p, _)| p.clone())
}

/// True when `needle` at `col` in `line` is a standalone Turtle token (not a substring).
pub fn is_token_match_at(line: &str, needle: &str, col: usize) -> bool {
    if !line[col..].starts_with(needle) {
        return false;
    }
    is_safe_replacement_boundary(line, col, col + needle.len())
}

/// True when a byte is a valid continuation of an IRI or Turtle PN_LOCAL token.
fn is_iri_continuation(b: u8) -> bool {
    b.is_ascii_alphanumeric()
        || matches!(b, b'_' | b'-' | b'.' | b'~' | b':' | b'#' | b'/' | b'%' | b'?' | b'=' | b'&')
}

pub fn is_safe_replacement_boundary(text: &str, start: usize, end: usize) -> bool {
    let before = text.as_bytes().get(start.wrapping_sub(1)).copied();
    let after = text.as_bytes().get(end).copied();
    let ok_before = before.is_none_or(|b| !is_iri_continuation(b));
    let ok_after = after.is_none_or(|b| !is_iri_continuation(b));
    ok_before && ok_after
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_iri_in_angle_brackets() {
        let ttl = "@prefix ex: <http://example.org#> .\nex:Foo a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, hunks) =
            replace_iri_in_text(ttl, "http://example.org#Foo", "http://example.org#Bar", &ns);
        assert!(!hunks.is_empty());
        assert!(out.contains("ex:Bar"));
        assert!(!out.contains("ex:Foo"));
    }

    #[test]
    fn replace_iri_does_not_corrupt_slash_extended_iri() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   ex:Role a owl:Class .\n\
                   <http://example.org#Person/role> a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Agent", &ns);
        assert!(out.contains("<http://example.org#Person/role>"));
        assert!(!out.contains("<http://example.org#Agent/role>"));
    }

    #[test]
    fn replace_iri_does_not_corrupt_underscore_extended_iri() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   ex:Person a owl:Class .\n\
                   ex:Person_extra a owl:Class .\n\
                   <http://example.org#Person-v2> a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Agent", &ns);
        assert!(out.contains("ex:Agent a owl:Class"));
        assert!(out.contains("ex:Person_extra a owl:Class"));
        assert!(out.contains("<http://example.org#Person-v2>"));
        assert!(!out.contains("ex:Agent_extra"));
        assert!(!out.contains("Agent-v2"));
    }

    #[test]
    fn replace_iri_skips_comments_and_string_literals() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   # see http://example.org#Person\n\
                   ex:Person a owl:Class ;\n\
                       rdfs:comment \"see http://example.org#Person\" .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Agent", &ns);
        assert!(out.contains("# see http://example.org#Person"));
        assert!(out.contains("\"see http://example.org#Person\""));
        assert!(out.contains("ex:Agent"));
        assert!(!out.contains("ex:Person a"));
    }

    #[test]
    fn replace_iri_skips_single_quoted_literals() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
                   @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
                   ex:Doc rdfs:comment 'Documents http://example.org#Person usage' .\n\
                   ex:Person a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Human", &ns);
        assert!(out.contains("'Documents http://example.org#Person usage'"));
        assert!(out.contains("ex:Human a owl:Class"));
        assert!(!out.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn replace_iri_skips_long_double_quoted_literals() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
                   @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
                   ex:Doc rdfs:comment \"\"\"See http://example.org#Person here\"\"\" .\n\
                   ex:Person a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Human", &ns);
        assert!(out.contains("http://example.org#Person here"));
        assert!(out.contains("ex:Human a owl:Class"));
    }

    #[test]
    fn replace_iri_skips_long_single_quoted_literals() {
        let ttl = "@prefix ex: <http://example.org#> .\n\
                   @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
                   @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
                   ex:Doc rdfs:comment '''See http://example.org#Person here''' .\n\
                   ex:Person a owl:Class .\n";
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org#".to_string())]);
        let (out, _) =
            replace_iri_in_text(ttl, "http://example.org#Person", "http://example.org#Human", &ns);
        assert!(out.contains("http://example.org#Person here"));
        assert!(out.contains("ex:Human a owl:Class"));
    }

    #[test]
    fn replace_iri_renames_default_prefix_curie() {
        let ttl = concat!(
            "@prefix : <http://example.org/org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            ":Person a owl:Class .\n"
        );
        let ns = BTreeMap::from([("".to_string(), "http://example.org/org#".to_string())]);
        let (out, hunks) = replace_iri_in_text(
            ttl,
            "http://example.org/org#Person",
            "http://example.org/org#Human",
            &ns,
        );
        assert!(!hunks.is_empty());
        assert!(out.contains(":Human a owl:Class"));
        assert!(!out.contains(":Person a owl:Class"));
    }

    #[test]
    fn replace_iri_uses_absolute_form_when_target_namespace_undeclared() {
        let ttl = concat!(
            "@prefix ex: <http://example.org/v1#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "ex:Person a owl:Class .\n"
        );
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org/v1#".to_string())]);
        let (out, hunks) = replace_iri_in_text(
            ttl,
            "http://example.org/v1#Person",
            "http://example.org/v2#Human",
            &ns,
        );
        assert!(!hunks.is_empty());
        assert!(
            out.contains("<http://example.org/v2#Human>"),
            "must not reuse ex: for undeclared v2 namespace: {out}"
        );
        assert!(!out.contains("ex:Human"), "ex:Human would still expand under v1#: {out}");
        assert!(!out.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn replace_iri_rewrites_same_local_name_across_namespaces() {
        let ttl = concat!(
            "@prefix ex: <http://example.org/v1#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "ex:Person a owl:Class .\n"
        );
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org/v1#".to_string())]);
        let (out, _) = replace_iri_in_text(
            ttl,
            "http://example.org/v1#Person",
            "http://example.org/v2#Person",
            &ns,
        );
        assert!(
            out.contains("<http://example.org/v2#Person>"),
            "same local name must still rewrite when namespace changes: {out}"
        );
        assert!(!out.contains("ex:Person a owl:Class"));
    }
}
