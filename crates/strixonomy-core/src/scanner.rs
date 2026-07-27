use crate::error::{Result, StrixonomyError};
use crate::limits::{MAX_FILE_BYTES, MAX_SCAN_FILES, MAX_SCAN_WALK_ENTRIES};
use crate::model::OntologyFormat;
use crate::path_jail::{canonical_workspace_root, is_path_within};
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const ONTOLOGY_EXTENSIONS: &[&str] =
    &["ttl", "rdf", "owl", "owx", "jsonld", "json-ld", "nt", "nq", "trig", "obo"];

#[derive(Debug, Clone)]
pub struct OntologyFile {
    pub path: PathBuf,
    pub format: OntologyFormat,
    pub content_hash: String,
    pub modified_time: u64,
    pub size_bytes: u64,
}

pub struct WorkspaceScanner {
    root: PathBuf,
    canonical_root: PathBuf,
}

impl WorkspaceScanner {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let canonical_root = canonical_workspace_root(&root).unwrap_or_else(|_| root.clone());
        Self { root, canonical_root }
    }

    pub fn canonical_root(&self) -> &Path {
        &self.canonical_root
    }

    pub fn scan(&self) -> Result<Vec<OntologyFile>> {
        if !self.root.exists() {
            return Err(StrixonomyError::Scanner(format!(
                "workspace path does not exist: {}",
                self.root.display()
            )));
        }

        let mut files = Vec::new();
        let mut walked = 0usize;
        let walker = WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .follow_links(false)
            .build();

        for entry in walker {
            walked += 1;
            if walked > MAX_SCAN_WALK_ENTRIES {
                return Err(StrixonomyError::Scanner(format!(
                    "workspace walk exceeds maximum of {MAX_SCAN_WALK_ENTRIES} entries"
                )));
            }
            if files.len() >= MAX_SCAN_FILES {
                return Err(StrixonomyError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_SCAN_FILES} ontology files"
                )));
            }

            let entry = entry.map_err(|e| StrixonomyError::Scanner(e.to_string()))?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if path.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false) {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ONTOLOGY_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()) {
                    let canonical =
                        path.canonicalize().map_err(|e| StrixonomyError::Scanner(e.to_string()))?;
                    if !is_path_within(&self.canonical_root, &canonical) {
                        continue;
                    }
                    files.push(self.describe_file(&canonical)?);
                }
            }
        }

        files.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(files)
    }

    /// Describe a single ontology file on disk (hash, format, mtime).
    pub fn describe_path(&self, path: &Path) -> Result<OntologyFile> {
        let canonical = path.canonicalize().map_err(|e| StrixonomyError::Scanner(e.to_string()))?;
        if !is_path_within(&self.canonical_root, &canonical) {
            return Err(StrixonomyError::Scanner(format!(
                "path outside workspace: {}",
                path.display()
            )));
        }
        self.describe_file(&canonical)
    }

    fn describe_file(&self, path: &Path) -> Result<OntologyFile> {
        let metadata = fs::metadata(path)?;
        let size_bytes = metadata.len();
        if size_bytes > MAX_FILE_BYTES {
            return Err(StrixonomyError::Scanner(format!(
                "file exceeds size limit ({} bytes > {MAX_FILE_BYTES}): {}",
                size_bytes,
                path.display()
            )));
        }

        let content = crate::io::read_file_capped(path, MAX_FILE_BYTES)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let content_hash = hex::encode(hasher.finalize());

        let modified_time = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or_default();

        Ok(OntologyFile {
            path: path.to_path_buf(),
            format: OntologyFormat::from_extension(ext),
            content_hash,
            modified_time,
            size_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn scan_finds_ontology_files() {
        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("example.ttl");
        let mut f = fs::File::create(&ttl).unwrap();
        writeln!(f, "@prefix ex: <http://example.org/> .").unwrap();

        let txt = dir.path().join("readme.txt");
        fs::write(txt, "not ontology").unwrap();

        let scanner = WorkspaceScanner::new(dir.path());
        let files = scanner.scan().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].format, OntologyFormat::Turtle);
        assert!(!files[0].content_hash.is_empty());
    }

    #[test]
    fn scan_finds_owx_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("example.owx"),
            r#"<?xml version="1.0"?><Ontology xmlns="http://www.w3.org/2002/07/owl#" ontologyIRI="http://ex.org"/>"#,
        )
        .unwrap();
        let scanner = WorkspaceScanner::new(dir.path());
        let files = scanner.scan().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].format, OntologyFormat::OwlXml);
    }

    #[test]
    fn skips_symlinked_ontology_files() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let secret = outside.path().join("secret.ttl");
        fs::write(&secret, "@prefix ex: <http://ex/> .").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let link = dir.path().join("linked.ttl");
            symlink(&secret, &link).unwrap();
            let scanner = WorkspaceScanner::new(dir.path());
            let files = scanner.scan().unwrap();
            assert_eq!(files.len(), 0);
        }
    }

    #[test]
    fn skips_files_in_sibling_prefix_directory() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("ws");
        fs::create_dir_all(&root).unwrap();
        let sibling = dir.path().join("ws_extra");
        fs::create_dir_all(&sibling).unwrap();
        fs::write(sibling.join("secret.ttl"), "@prefix ex: <http://ex/> .").unwrap();

        let scanner = WorkspaceScanner::new(&root);
        let files = scanner.scan().unwrap();
        assert_eq!(files.len(), 0);
    }
}
