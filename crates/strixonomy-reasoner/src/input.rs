use crate::error::{ReasonerError, Result};
use crate::hierarchy::asserted_hierarchy_from_ontology;
use ontologos_bridge::{core_to_triples_all, merge_triples_into_ontology};
use ontologos_core::Ontology;
use ontologos_parser::load_ontology;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strixonomy_catalog::ClassHierarchy;
use strixonomy_core::{OntologyFile, OntologyFormat, WorkspaceScanner};
use strixonomy_parser::{parse_ontology_file, parse_ontology_text, serialize_quads_turtle};

#[derive(Debug, Clone)]
pub struct ReasonerInput {
    pub workspace: PathBuf,
    pub content_hash: String,
    pub ontology: Ontology,
    pub asserted_hierarchy: ClassHierarchy,
    pub document_overrides: HashMap<PathBuf, String>,
}

pub struct WorkspaceInputLoader {
    workspace: PathBuf,
    scan_roots: Vec<PathBuf>,
    document_overrides: HashMap<PathBuf, String>,
}

impl WorkspaceInputLoader {
    pub fn new(workspace: impl Into<PathBuf>) -> Self {
        Self {
            workspace: workspace.into(),
            scan_roots: Vec::new(),
            document_overrides: HashMap::new(),
        }
    }

    pub fn document_overrides(mut self, overrides: HashMap<PathBuf, String>) -> Self {
        self.document_overrides = overrides;
        self
    }

    /// Additional workspace roots to scan (multi-root), matching catalog `scan_roots`.
    pub fn scan_roots(mut self, roots: Vec<PathBuf>) -> Self {
        self.scan_roots = roots;
        self
    }

    pub fn load(&self) -> Result<ReasonerInput> {
        let scan_roots = strixonomy_catalog::IndexBuilder::effective_scan_roots(
            &self.workspace,
            &self.scan_roots,
        );
        let mut files: Vec<OntologyFile> = Vec::new();
        for root in &scan_roots {
            let scanner = WorkspaceScanner::new(root);
            for file in scanner.scan()? {
                if !files.iter().any(|f| paths_equal(&f.path, &file.path)) {
                    files.push(file);
                }
            }
        }

        let mut hasher = Sha256::new();
        let mut ontology = Ontology::new();

        for file in &files {
            let override_text = self.document_override_text(&file.path).cloned();
            let loaded = if let Some(ref text) = override_text {
                // Hash override body so open-buffer edits invalidate the reasoner cache.
                hasher.update(file.path.to_string_lossy().as_bytes());
                hasher.update(text.as_bytes());
                load_workspace_file(&file.path, file.format, Some(text), file)?
            } else {
                hasher.update(file.content_hash.as_bytes());
                load_workspace_file(&file.path, file.format, None, file)?
            };
            merge_ontology(&mut ontology, loaded)?;
            if let Some(ref text) = override_text {
                let _ = crate::swrl_run::inject_swrl_from_turtle(&mut ontology, text);
            } else if matches!(file.format, OntologyFormat::Turtle) {
                if let Ok(text) = std::fs::read_to_string(&file.path) {
                    let _ = crate::swrl_run::inject_swrl_from_turtle(&mut ontology, &text);
                }
            }
        }

        for (path, text) in &self.document_overrides {
            if files.iter().any(|f| paths_equal(&f.path, path)) {
                continue;
            }
            hasher.update(path.to_string_lossy().as_bytes());
            hasher.update(text.as_bytes());
            let format = OntologyFormat::from_extension(
                path.extension().and_then(|e| e.to_str()).unwrap_or("ttl"),
            );
            let file_stub = OntologyFile {
                path: path.clone(),
                format,
                content_hash: String::new(),
                modified_time: 0,
                size_bytes: text.len() as u64,
            };
            let loaded = load_workspace_file(path, format, Some(text), &file_stub)?;
            merge_ontology(&mut ontology, loaded)?;
            let _ = crate::swrl_run::inject_swrl_from_turtle(&mut ontology, text);
        }

        let asserted_hierarchy = asserted_hierarchy_from_ontology(&ontology);

        Ok(ReasonerInput {
            workspace: self.workspace.clone(),
            content_hash: hex::encode(hasher.finalize()),
            ontology,
            asserted_hierarchy,
            document_overrides: self.document_overrides.clone(),
        })
    }

    fn document_override_text(&self, path: &Path) -> Option<&String> {
        if let Some(text) = self.document_overrides.get(path) {
            return Some(text);
        }
        let canonical = path.canonicalize().ok();
        if let Some(ref canon) = canonical {
            if let Some(text) = self.document_overrides.get(canon) {
                return Some(text);
            }
        }
        // Match overrides stored under a different spelling of the same path
        // (e.g. /var vs /private/var on macOS).
        self.document_overrides.iter().find_map(|(key, text)| {
            if paths_equal(key, path) {
                Some(text)
            } else {
                None
            }
        })
    }
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

fn load_workspace_file(
    path: &Path,
    format: OntologyFormat,
    override_text: Option<&str>,
    file: &OntologyFile,
) -> Result<Ontology> {
    if format == OntologyFormat::Obo {
        return load_obo_as_ontology(path, override_text, file);
    }
    if let Some(text) = override_text {
        return load_ontology_from_temp(path, text);
    }
    load_ontology(path)
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })
}

fn load_obo_as_ontology(
    path: &Path,
    override_text: Option<&str>,
    file: &OntologyFile,
) -> Result<Ontology> {
    let parsed = if let Some(text) = override_text {
        parse_ontology_text(path, OntologyFormat::Obo, "reasoner", text, text.as_bytes())
    } else {
        parse_ontology_file(
            path,
            OntologyFormat::Obo,
            "reasoner",
            &file.content_hash,
            file.modified_time,
        )
    }
    .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })?;

    if parsed.quads().is_empty() {
        return Err(ReasonerError::Load {
            path: path.to_path_buf(),
            message: "OBO file produced no RDF quads".to_string(),
        });
    }

    let turtle = serialize_quads_turtle(parsed.quads())
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })?;
    load_ontology_from_temp_with_suffix(path, &turtle, "ttl")
}

fn load_ontology_from_temp(path: &Path, text: &str) -> Result<Ontology> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("ttl");
    load_ontology_from_temp_with_suffix(path, text, ext)
}

fn load_ontology_from_temp_with_suffix(path: &Path, text: &str, ext: &str) -> Result<Ontology> {
    use strixonomy_core::MAX_FILE_BYTES;
    if text.len() as u64 > MAX_FILE_BYTES {
        return Err(ReasonerError::Load {
            path: path.to_path_buf(),
            message: format!("document exceeds maximum size of {MAX_FILE_BYTES} bytes"),
        });
    }
    let tmp = tempfile::Builder::new()
        .suffix(&format!(".{ext}"))
        .tempfile()
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })?;
    std::fs::write(tmp.path(), text)
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })?;
    load_ontology(tmp.path())
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })
}

fn merge_ontology(target: &mut Ontology, source: Ontology) -> Result<()> {
    let triples =
        core_to_triples_all(&source).map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    merge_triples_into_ontology(target, &triples, &[])
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{classify, ReasonerId};
    use std::fs;
    use strixonomy_catalog::IndexBuilder;

    #[test]
    fn content_hash_changes_when_override_differs_from_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ex.ttl");
        fs::write(
            &path,
            "@prefix ex: <http://ex#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\n",
        )
        .unwrap();

        let disk_input = WorkspaceInputLoader::new(dir.path()).load().expect("disk load");

        let mut overrides = HashMap::new();
        overrides.insert(
            path.clone(),
            "@prefix ex: <http://ex#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:A a owl:Class .\nex:B a owl:Class .\n"
                .to_string(),
        );
        let override_input = WorkspaceInputLoader::new(dir.path())
            .document_overrides(overrides)
            .load()
            .expect("override load");

        assert_ne!(
            disk_input.content_hash, override_input.content_hash,
            "open-buffer overrides must change reasoner content_hash"
        );
    }

    #[test]
    fn buffer_subclass_axiom_not_reported_as_new_inference() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ex.ttl");
        fs::write(
            &path,
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             ex:A a owl:Class .\n\
             ex:C a owl:Class ; rdfs:subClassOf ex:A .\n",
        )
        .unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
        assert!(
            !catalog.class_hierarchy().edges.iter().any(|e| e.child.ends_with("D")),
            "stale catalog must not yet include ex:D"
        );

        let mut overrides = HashMap::new();
        overrides.insert(
            path,
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             ex:A a owl:Class .\n\
             ex:C a owl:Class ; rdfs:subClassOf ex:A .\n\
             ex:D a owl:Class ; rdfs:subClassOf ex:C .\n"
                .to_string(),
        );

        let input = WorkspaceInputLoader::new(dir.path())
            .document_overrides(overrides)
            .load()
            .expect("override load");

        assert!(
            input
                .asserted_hierarchy
                .edges
                .iter()
                .any(|e| e.child.ends_with("D") && e.parent.ends_with("C")),
            "asserted hierarchy must include buffer subclass axiom D ⊑ C"
        );

        let result = classify(ReasonerId::Rdfs, &input, false).expect("classify");
        assert!(
            !result
                .new_inferences
                .iter()
                .any(|e| e.child.ends_with("D") && e.parent.ends_with("C")),
            "buffer-authored D ⊑ C must not appear in new_inferences"
        );
    }

    #[test]
    fn asserted_hierarchy_matches_catalog_without_overrides() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ex.ttl");
        fs::write(
            &path,
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             ex:A a owl:Class .\n\
             ex:C a owl:Class ; rdfs:subClassOf ex:A .\n",
        )
        .unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
        let input = WorkspaceInputLoader::new(dir.path()).load().expect("load");

        let catalog_edges: std::collections::BTreeSet<_> = catalog
            .class_hierarchy()
            .edges
            .iter()
            .map(|e| (e.child.clone(), e.parent.clone()))
            .collect();
        let input_edges: std::collections::BTreeSet<_> = input
            .asserted_hierarchy
            .edges
            .iter()
            .map(|e| (e.child.clone(), e.parent.clone()))
            .collect();
        assert_eq!(catalog_edges, input_edges);
    }

    #[test]
    fn loads_minimal_obo_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.obo");
        fs::write(
            &path,
            "format-version: 1.2\nontology: test\n\n\
[Term]\n\
id: TEST:0000001\n\
name: child\n\
is_a: TEST:0000002 ! parent\n\n\
[Term]\n\
id: TEST:0000002\n\
name: parent\n",
        )
        .unwrap();

        let input = WorkspaceInputLoader::new(dir.path())
            .load()
            .expect("OBO workspace should load for reasoner");
        let triples = core_to_triples_all(&input.ontology).expect("triples");
        assert!(!triples.is_empty(), "OBO-derived ontology should contain triples");
    }
}
