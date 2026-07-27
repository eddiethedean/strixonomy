use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::{DiagnosticSeverity, EntityKind};
use strixonomy_plugin::{plugin_diagnostic, ValidatorPlugin};

pub const PLUGIN_ID: &str = "strixonomy.naming-validator";

#[derive(Debug, Clone)]
pub struct NamingValidatorPlugin {
    pub require_label: bool,
    pub iri_prefix: Option<String>,
}

impl Default for NamingValidatorPlugin {
    fn default() -> Self {
        Self { require_label: true, iri_prefix: None }
    }
}

impl NamingValidatorPlugin {
    pub fn from_config(require_label: bool, iri_prefix: Option<String>) -> Self {
        Self { require_label, iri_prefix }
    }
}

impl ValidatorPlugin for NamingValidatorPlugin {
    fn id(&self) -> &str {
        PLUGIN_ID
    }

    fn validate(
        &self,
        catalog: &OntologyCatalog,
        _workspace: &Path,
    ) -> Vec<strixonomy_core::Diagnostic> {
        let mut diagnostics = Vec::new();
        for entity in &catalog.data().entities {
            if self.require_label
                && matches!(
                    entity.kind,
                    EntityKind::Class
                        | EntityKind::ObjectProperty
                        | EntityKind::DataProperty
                        | EntityKind::Individual
                )
                && entity.labels.is_empty()
            {
                diagnostics.push(plugin_diagnostic(
                    PLUGIN_ID,
                    "missing_label",
                    DiagnosticSeverity::Warning,
                    format!("Entity '{}' has no rdfs:label", entity.iri),
                    entity_file(catalog, entity),
                    Some(entity.iri.clone()),
                ));
            }
            if let Some(prefix) = &self.iri_prefix {
                if !entity.iri.starts_with(prefix) {
                    diagnostics.push(plugin_diagnostic(
                        PLUGIN_ID,
                        "iri_prefix",
                        DiagnosticSeverity::Warning,
                        format!(
                            "Entity IRI '{}' does not start with required prefix '{prefix}'",
                            entity.iri
                        ),
                        entity_file(catalog, entity),
                        Some(entity.iri.clone()),
                    ));
                }
            }
        }
        diagnostics
    }
}

fn entity_file(catalog: &OntologyCatalog, entity: &strixonomy_core::Entity) -> PathBuf {
    catalog
        .data()
        .documents
        .iter()
        .find(|d| d.id == entity.ontology_id)
        .map(|d| d.path.clone())
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use strixonomy_catalog::IndexBuilder;

    #[test]
    fn flags_class_without_label() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("test.ttl"),
            r#"
@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:A a owl:Class .
"#,
        )
        .unwrap();
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
        let plugin = NamingValidatorPlugin::default();
        let diags = plugin.validate(&catalog, dir.path());
        assert!(!diags.is_empty());
        assert!(diags.iter().any(|d| d.plugin_code.as_deref() == Some("missing_label")));
    }
}
