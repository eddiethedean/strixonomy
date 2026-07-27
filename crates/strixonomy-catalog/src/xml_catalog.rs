//! Oasis / Protégé `catalog-v001.xml` IRI → local path redirects.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use strixonomy_core::{is_path_within, read_to_string_capped, MAX_FILE_BYTES};
use thiserror::Error;

/// Maximum `nextCatalog` nesting depth (cycle-safe bound).
const MAX_NEXTCATALOG_DEPTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XmlCatalogError {
    #[error("io: {0}")]
    Io(String),
    #[error("parse: {0}")]
    Parse(String),
}

/// Parsed XML Catalog redirect table (Protégé `catalog-v001.xml` subset).
#[derive(Debug, Clone, Default)]
pub struct XmlCatalog {
    /// Absolute path of the catalog file (used to resolve relative uris).
    pub catalog_path: PathBuf,
    /// Exact IRI (`name`) → redirect URI string (may be relative).
    pub uri_entries: BTreeMap<String, String>,
    /// Prefix rewrite rules: start string → replacement prefix.
    pub rewrite_entries: Vec<(String, String)>,
}

impl XmlCatalog {
    /// Resolve an ontology import IRI to a local filesystem path when possible.
    pub fn resolve(&self, iri: &str) -> Option<PathBuf> {
        let redirect = self.resolve_uri(iri)?;
        Some(self.absolutize(&redirect))
    }

    /// Resolve to the redirect URI string (relative or absolute).
    pub fn resolve_uri(&self, iri: &str) -> Option<String> {
        if let Some(u) = self.uri_entries.get(iri) {
            return Some(u.clone());
        }
        // Longest rewriteURI prefix wins
        let mut best: Option<&(String, String)> = None;
        for rule in &self.rewrite_entries {
            if iri.starts_with(&rule.0) && best.as_ref().is_none_or(|b| rule.0.len() > b.0.len()) {
                best = Some(rule);
            }
        }
        best.map(|(start, replace)| format!("{}{}", replace, &iri[start.len()..]))
    }

    fn absolutize(&self, redirect: &str) -> PathBuf {
        let p = Path::new(redirect);
        if p.is_absolute() {
            return p.to_path_buf();
        }
        if let Some(rest) = redirect.strip_prefix("file://") {
            return PathBuf::from(rest);
        }
        let parent = self.catalog_path.parent().unwrap_or_else(|| Path::new("."));
        parent.join(redirect)
    }
}

/// Parse a Protégé/Oasis catalog XML document.
///
/// Nested `nextCatalog` entries are jailed to the catalog file's parent directory
/// (use [`parse_xml_catalog_in_workspace`] when a workspace root is known).
pub fn parse_xml_catalog(path: &Path, xml: &str) -> Result<XmlCatalog, XmlCatalogError> {
    let jail = path.parent().map(Path::to_path_buf);
    parse_xml_catalog_inner(path, xml, jail.as_deref(), &mut Vec::new(), &mut BTreeSet::new(), 0)
}

/// Parse a catalog, jailing nested `nextCatalog` loads under `workspace_root`.
pub fn parse_xml_catalog_in_workspace(
    path: &Path,
    xml: &str,
    workspace_root: &Path,
) -> Result<XmlCatalog, XmlCatalogError> {
    parse_xml_catalog_inner(
        path,
        xml,
        Some(workspace_root),
        &mut Vec::new(),
        &mut BTreeSet::new(),
        0,
    )
}

fn parse_xml_catalog_inner(
    path: &Path,
    xml: &str,
    jail_root: Option<&Path>,
    stack: &mut Vec<PathBuf>,
    loaded: &mut BTreeSet<PathBuf>,
    depth: usize,
) -> Result<XmlCatalog, XmlCatalogError> {
    if depth > MAX_NEXTCATALOG_DEPTH {
        return Err(XmlCatalogError::Parse(format!(
            "nextCatalog nesting exceeds limit ({MAX_NEXTCATALOG_DEPTH}) at {}",
            path.display()
        )));
    }
    let identity = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if stack.iter().any(|p| p == &identity) {
        return Err(XmlCatalogError::Parse(format!(
            "nextCatalog cycle detected at {}",
            path.display()
        )));
    }
    if loaded.contains(&identity) {
        // Already fully processed in this load — skip without error (DAG / diamond).
        return Ok(XmlCatalog { catalog_path: path.to_path_buf(), ..Default::default() });
    }
    stack.push(identity.clone());

    let mut catalog = XmlCatalog { catalog_path: path.to_path_buf(), ..Default::default() };
    // Prefer xml:base on catalog/group when present
    let mut active_base: Option<String> = None;

    // Walk uri / rewriteURI tags with a lightweight scanner
    for (tag, attrs) in iter_empty_elements(xml) {
        let local = local_name(&tag);
        if local == "catalog" || local == "group" {
            if let Some(b) = attrs.get("xml:base").or_else(|| attrs.get("base")) {
                if !b.is_empty() {
                    active_base = Some(b.clone());
                }
            }
            continue;
        }
        if local == "uri" {
            let Some(name) = attrs.get("name").cloned() else {
                continue;
            };
            let Some(uri) = attrs.get("uri").cloned() else {
                continue;
            };
            let uri = apply_xml_base(&uri, active_base.as_deref());
            catalog.uri_entries.insert(name, uri);
        } else if local == "rewriteURI" {
            let Some(start) = attrs.get("uriStartString").cloned() else {
                continue;
            };
            let Some(rewrite) = attrs.get("rewritePrefix").cloned() else {
                continue;
            };
            catalog.rewrite_entries.push((start, rewrite));
        } else if local == "nextCatalog" {
            if let Some(next) = attrs.get("catalog") {
                let next_path = catalog.absolutize(next);
                load_nested_catalog(&mut catalog, &next_path, jail_root, stack, loaded, depth + 1)?;
            }
        }
    }
    stack.pop();
    loaded.insert(identity);
    Ok(catalog)
}

fn load_nested_catalog(
    catalog: &mut XmlCatalog,
    next_path: &Path,
    jail_root: Option<&Path>,
    stack: &mut Vec<PathBuf>,
    loaded: &mut BTreeSet<PathBuf>,
    depth: usize,
) -> Result<(), XmlCatalogError> {
    if !next_path.is_file() {
        return Ok(());
    }
    let resolved = next_path
        .canonicalize()
        .map_err(|e| XmlCatalogError::Io(format!("{}: {e}", next_path.display())))?;
    if let Some(root) = jail_root {
        let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        if !is_path_within(&root, &resolved) {
            return Err(XmlCatalogError::Parse(format!(
                "nextCatalog path escapes workspace jail ({}): {}",
                root.display(),
                next_path.display()
            )));
        }
    }
    if loaded.contains(&resolved) {
        return Ok(());
    }
    let text = read_to_string_capped(&resolved, MAX_FILE_BYTES)
        .map_err(|e| XmlCatalogError::Io(e.to_string()))?;
    let nested = parse_xml_catalog_inner(&resolved, &text, jail_root, stack, loaded, depth)?;
    for (k, v) in nested.uri_entries {
        catalog.uri_entries.entry(k).or_insert(v);
    }
    catalog.rewrite_entries.extend(nested.rewrite_entries);
    Ok(())
}

/// Load and parse a catalog file from disk (size-capped).
pub fn load_xml_catalog(path: &Path) -> Result<XmlCatalog, XmlCatalogError> {
    let text = read_to_string_capped(path, MAX_FILE_BYTES)
        .map_err(|e| XmlCatalogError::Io(e.to_string()))?;
    parse_xml_catalog(path, &text)
}

/// Discover `catalog-v001.xml` / `catalog-*.xml` under `root` (non-recursive first, then depth 1).
pub fn discover_workspace_catalogs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let candidates = [root.join("catalog-v001.xml"), root.join("catalog.xml")];
    for c in candidates {
        if c.is_file() {
            out.push(c);
        }
    }
    if let Ok(rd) = std::fs::read_dir(root) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("catalog-") && name.ends_with(".xml") && !out.contains(&p) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

/// Merge all discovered catalogs under a workspace root.
pub fn load_workspace_xml_catalogs(root: &Path) -> Result<XmlCatalog, XmlCatalogError> {
    let paths = discover_workspace_catalogs(root);
    let mut merged =
        XmlCatalog { catalog_path: root.join("catalog-v001.xml"), ..Default::default() };
    for path in paths {
        let text = read_to_string_capped(&path, MAX_FILE_BYTES)
            .map_err(|e| XmlCatalogError::Io(e.to_string()))?;
        let cat = parse_xml_catalog_in_workspace(&path, &text, root)?;
        merged.catalog_path = path;
        for (k, v) in cat.uri_entries {
            merged.uri_entries.insert(k, v);
        }
        merged.rewrite_entries.extend(cat.rewrite_entries);
    }
    Ok(merged)
}

fn apply_xml_base(uri: &str, base: Option<&str>) -> String {
    let p = Path::new(uri);
    if p.is_absolute() || uri.contains("://") {
        return uri.to_string();
    }
    if let Some(b) = base {
        if b.is_empty() {
            return uri.to_string();
        }
        if b.ends_with('/') {
            format!("{b}{uri}")
        } else {
            format!("{b}/{uri}")
        }
    } else {
        uri.to_string()
    }
}

fn local_name(tag: &str) -> &str {
    tag.rsplit([':', '}']).next().unwrap_or(tag)
}

/// Yield (tag_name, attrs) for empty-element or start tags.
fn iter_empty_elements(xml: &str) -> Vec<(String, BTreeMap<String, String>)> {
    let mut out = Vec::new();
    let bytes = xml.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        if i + 1 < bytes.len()
            && (bytes[i + 1] == b'!' || bytes[i + 1] == b'?' || bytes[i + 1] == b'/')
        {
            // skip comments / decls / end tags — advance to '>'
            if let Some(rel) = xml[i..].find('>') {
                i += rel + 1;
            } else {
                break;
            }
            continue;
        }
        let start = i + 1;
        let Some(end_rel) = xml[start..].find('>') else {
            break;
        };
        let end = start + end_rel;
        let mut inner = &xml[start..end];
        inner = inner.trim_end_matches('/').trim();
        let (name, rest) = match inner.split_once(|c: char| c.is_whitespace()) {
            Some((n, r)) => (n, r),
            None => (inner, ""),
        };
        let attrs = parse_attrs(rest);
        out.push((name.to_string(), attrs));
        i = end + 1;
    }
    out
}

fn parse_attrs(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let mut rest = s;
    while let Some(eq) = rest.find('=') {
        let key = rest[..eq]
            .trim()
            .rsplit_once(char::is_whitespace)
            .map(|(_, k)| k)
            .unwrap_or(rest[..eq].trim());
        let after = rest[eq + 1..].trim_start();
        let (val, next) = if let Some(stripped) = after.strip_prefix('"') {
            if let Some(e) = stripped.find('"') {
                (&stripped[..e], &stripped[e + 1..])
            } else {
                break;
            }
        } else if let Some(stripped) = after.strip_prefix('\'') {
            if let Some(e) = stripped.find('\'') {
                (&stripped[..e], &stripped[e + 1..])
            } else {
                break;
            }
        } else {
            break;
        };
        if !key.is_empty() {
            map.insert(key.to_string(), val.to_string());
        }
        rest = next;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_uri_redirect() {
        let xml = r#"<?xml version="1.0"?>
<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog" prefer="public">
  <group id="Folder" prefer="public" xml:base="">
    <uri name="http://example.org/ont/pizza" uri="pizza.ttl"/>
  </group>
</catalog>"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog-v001.xml");
        std::fs::write(&path, xml).unwrap();
        let cat = parse_xml_catalog(&path, xml).unwrap();
        assert_eq!(cat.resolve_uri("http://example.org/ont/pizza").as_deref(), Some("pizza.ttl"));
        let resolved = cat.resolve("http://example.org/ont/pizza").unwrap();
        assert_eq!(resolved, dir.path().join("pizza.ttl"));
    }

    #[test]
    fn xml_base_prefixes_relative_uri() {
        let xml = r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <group xml:base="lib/">
    <uri name="http://example.org/a" uri="a.ttl"/>
  </group>
</catalog>"#;
        let path = Path::new("/tmp/catalog-v001.xml");
        let cat = parse_xml_catalog(path, xml).unwrap();
        assert_eq!(cat.resolve_uri("http://example.org/a").as_deref(), Some("lib/a.ttl"));
    }

    #[test]
    fn next_catalog_cycle_errors() {
        // #392
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.xml");
        let b = dir.path().join("b.xml");
        std::fs::write(
            &a,
            r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <nextCatalog catalog="b.xml"/>
</catalog>"#,
        )
        .unwrap();
        std::fs::write(
            &b,
            r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <nextCatalog catalog="a.xml"/>
</catalog>"#,
        )
        .unwrap();
        let text = std::fs::read_to_string(&a).unwrap();
        let err = parse_xml_catalog(&a, &text).expect_err("cycle");
        assert!(err.to_string().contains("cycle"), "{err}");
    }

    #[test]
    fn next_catalog_outside_workspace_errors() {
        // #393
        let dir = tempfile::tempdir().unwrap();
        let workspace = dir.path().join("ws");
        let outside = dir.path().join("outside");
        std::fs::create_dir_all(&workspace).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        let secret = outside.join("secret.xml");
        std::fs::write(
            &secret,
            r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <uri name="http://evil" uri="x.ttl"/>
</catalog>"#,
        )
        .unwrap();
        let catalog = workspace.join("catalog-v001.xml");
        std::fs::write(
            &catalog,
            format!(
                r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <nextCatalog catalog="{}"/>
</catalog>"#,
                secret.display()
            ),
        )
        .unwrap();
        let text = std::fs::read_to_string(&catalog).unwrap();
        let err = parse_xml_catalog_in_workspace(&catalog, &text, &workspace)
            .expect_err("must reject outside path");
        assert!(err.to_string().contains("escapes") || err.to_string().contains("jail"), "{err}");
    }

    #[test]
    fn next_catalog_relative_inside_workspace_ok() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = dir.path().join("ws");
        std::fs::create_dir_all(workspace.join("lib")).unwrap();
        std::fs::write(
            workspace.join("lib/nested.xml"),
            r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <uri name="http://example.org/n" uri="n.ttl"/>
</catalog>"#,
        )
        .unwrap();
        let catalog = workspace.join("catalog-v001.xml");
        std::fs::write(
            &catalog,
            r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <nextCatalog catalog="lib/nested.xml"/>
</catalog>"#,
        )
        .unwrap();
        let text = std::fs::read_to_string(&catalog).unwrap();
        let cat = parse_xml_catalog_in_workspace(&catalog, &text, &workspace).expect("nested");
        assert_eq!(cat.resolve_uri("http://example.org/n").as_deref(), Some("n.ttl"));
    }
}
