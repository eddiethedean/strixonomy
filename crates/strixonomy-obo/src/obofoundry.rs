//! OBO Foundry registry JSON model (Protégé Wave 3 port).
//!
//! Parses the OBO Foundry registry shape used by Protégé's `OboFoundry*` types.
//! Does **not** fetch live HTTP — tests use vendored fixtures.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OboFoundryError {
    #[error("invalid OBO Foundry registry JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, OboFoundryError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OboFoundryContact {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub github: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OboFoundryLicense {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub logo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OboFoundryEntry {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub activity_status: Option<String>,
    #[serde(default)]
    pub ontology_purl: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub contact: Option<OboFoundryContact>,
    #[serde(default)]
    pub license: Option<OboFoundryLicense>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RegistryFile {
    #[serde(default)]
    ontologies: Vec<OboFoundryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OboFoundryRegistry {
    ontologies: Vec<OboFoundryEntry>,
    by_id: BTreeMap<String, usize>,
}

impl OboFoundryRegistry {
    pub fn empty() -> Self {
        Self { ontologies: Vec::new(), by_id: BTreeMap::new() }
    }

    pub fn from_entries(ontologies: Vec<OboFoundryEntry>) -> Self {
        let mut by_id = BTreeMap::new();
        for (i, e) in ontologies.iter().enumerate() {
            by_id.insert(e.id.clone(), i);
        }
        Self { ontologies, by_id }
    }

    pub fn ontologies(&self) -> &[OboFoundryEntry] {
        &self.ontologies
    }

    pub fn get(&self, id: &str) -> Option<&OboFoundryEntry> {
        self.by_id.get(id).map(|&i| &self.ontologies[i])
    }

    pub fn len(&self) -> usize {
        self.ontologies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ontologies.is_empty()
    }
}

/// Parse an OBO Foundry registry JSON document (`{"ontologies":[...]}`).
pub fn parse_registry_json(bytes: &[u8]) -> Result<OboFoundryRegistry> {
    let file: RegistryFile = serde_json::from_slice(bytes)?;
    Ok(OboFoundryRegistry::from_entries(file.ontologies))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_registry() {
        let json = br#"{
            "ontologies": [
                {
                    "id": "bfo",
                    "title": "Basic Formal Ontology",
                    "activity_status": "active",
                    "ontology_purl": "http://purl.obolibrary.org/obo/bfo.owl",
                    "contact": {"email": "a@b.c", "label": "Alice"},
                    "license": {"label": "CC-BY", "url": "http://creativecommons.org/licenses/by/4.0/"}
                }
            ]
        }"#;
        let reg = parse_registry_json(json).expect("parse");
        assert_eq!(reg.len(), 1);
        let e = reg.get("bfo").expect("bfo");
        assert_eq!(e.title.as_deref(), Some("Basic Formal Ontology"));
        assert_eq!(e.contact.as_ref().unwrap().label.as_deref(), Some("Alice"));
        assert_eq!(e.license.as_ref().unwrap().label.as_deref(), Some("CC-BY"));
    }

    #[test]
    fn empty_registry() {
        let reg = parse_registry_json(br#"{"ontologies":[]}"#).unwrap();
        assert!(reg.is_empty());
        assert!(reg.get("go").is_none());
    }
}
