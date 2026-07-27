use crate::error::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strixonomy_core::{read_to_string_capped, MAX_FILE_BYTES};

/// Read Turtle source for a document, preferring unsaved LSP buffer overrides.
pub fn read_source_text(path: &Path, overrides: &HashMap<PathBuf, String>) -> Result<String> {
    if let Some(text) = overrides.get(path) {
        return Ok(text.clone());
    }
    if let Ok(canonical) = path.canonicalize() {
        if let Some(text) = overrides.get(&canonical) {
            return Ok(text.clone());
        }
    }
    read_to_string_capped(path, MAX_FILE_BYTES).map_err(Into::into)
}
